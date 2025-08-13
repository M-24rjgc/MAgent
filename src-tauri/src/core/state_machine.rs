use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::{OptimizationSession, SessionState};

/// 状态机控制器
pub struct StateMachine {
    current_state: SessionState,
    session_id: Uuid,
}

impl StateMachine {
    pub fn new(session_id: Uuid) -> Self {
        Self {
            current_state: SessionState::Initializing,
            session_id,
        }
    }

    pub fn current_state(&self) -> &SessionState {
        &self.current_state
    }

    pub async fn transition_to(&mut self, new_state: SessionState) -> Result<()> {
        tracing::info!(
            "State transition for session {}: {:?} -> {:?}",
            self.session_id,
            self.current_state,
            new_state
        );

        // Validate transition
        if self.is_valid_transition(&new_state) {
            self.current_state = new_state;
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Invalid state transition from {:?} to {:?}",
                self.current_state,
                new_state
            ))
        }
    }

    fn is_valid_transition(&self, new_state: &SessionState) -> bool {
        use SessionState::*;

        match (&self.current_state, new_state) {
            // From Initializing
            (Initializing, Clarifying) => true,
            (Initializing, Error(_)) => true,

            // From Clarifying
            (Clarifying, Clarified) => true,
            (Clarifying, Error(_)) => true,

            // From Clarified
            (Clarified, AdvIterating(_)) => true,
            (Clarified, Error(_)) => true,

            // From AdvIterating
            (AdvIterating(_), AdvIterating(_)) => true, // Allow iteration increments
            (AdvIterating(_), Verified) => true,
            (AdvIterating(_), Error(_)) => true,

            // From Verified
            (Verified, Formatting) => true,
            (Verified, Error(_)) => true,

            // From Formatting
            (Formatting, Done) => true,
            (Formatting, Error(_)) => true,

            // Error state can transition to any state (recovery)
            (Error(_), _) => true,

            // Done is final
            (Done, _) => false,

            // All other transitions are invalid
            _ => false,
        }
    }

    pub fn should_stop_clarification(&self, confidence: f64, min_confidence: f64) -> bool {
        confidence >= min_confidence
    }

    pub fn should_stop_iteration(&self, scores: &[f64], improvement_threshold: f64) -> bool {
        if scores.len() < 2 {
            return false;
        }

        let recent_improvements: Vec<f64> = scores
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect();

        // Stop if recent improvements are below threshold
        recent_improvements
            .iter()
            .rev()
            .take(3) // Check last 3 improvements
            .all(|&improvement| improvement < improvement_threshold)
    }
}

/// 停止条件配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopConditions {
    pub clarification_confidence_threshold: f64,
    pub max_clarification_rounds: u32,
    pub iteration_improvement_threshold: f64,
    pub max_iterations: u32,
    pub min_consecutive_improvements: u32,
}

impl Default for StopConditions {
    fn default() -> Self {
        Self {
            clarification_confidence_threshold: 0.8,
            max_clarification_rounds: 5,
            iteration_improvement_threshold: 0.01,
            max_iterations: 10,
            min_consecutive_improvements: 3,
        }
    }
}

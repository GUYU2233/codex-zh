//! Goal summary for the bare `/goal` command.

use super::*;
use crate::goal_display::format_goal_elapsed_seconds;
use crate::status::format_tokens_compact;

impl ChatWidget {
    pub(crate) fn show_goal_summary(&mut self, goal: AppThreadGoal) {
        self.add_plain_history_lines(goal_summary_lines(&goal));
    }

    pub(crate) fn on_thread_goal_cleared(&mut self, thread_id: &str) {
        if self
            .thread_id
            .is_some_and(|active_thread_id| active_thread_id.to_string() == thread_id)
        {
            self.current_goal_status = None;
            self.update_collaboration_mode_indicator();
        }
    }
}

fn goal_summary_lines(goal: &AppThreadGoal) -> Vec<Line<'static>> {
    let mut lines = vec![
        Line::from("目标".bold()),
        Line::from(vec![
            "状态：".dim(),
            goal_status_label(goal.status).to_string().into(),
        ]),
        Line::from(vec!["目标内容：".dim(), goal.objective.clone().into()]),
        Line::from(vec![
            "已用时间：".dim(),
            format_goal_elapsed_seconds(goal.time_used_seconds).into(),
        ]),
        Line::from(vec![
            "已用 Tokens：".dim(),
            format_tokens_compact(goal.tokens_used).into(),
        ]),
    ];
    if let Some(token_budget) = goal.token_budget {
        lines.push(Line::from(vec![
            "Token 预算：".dim(),
            format_tokens_compact(token_budget).into(),
        ]));
    }
    let command_hint = match goal.status {
        AppThreadGoalStatus::Active => "命令：/goal pause，/goal clear",
        AppThreadGoalStatus::Paused => "命令：/goal unpause，/goal clear",
        AppThreadGoalStatus::BudgetLimited | AppThreadGoalStatus::Complete => "命令：/goal clear",
    };
    lines.push(Line::default());
    lines.push(Line::from(command_hint.dim()));
    lines
}

fn goal_status_label(status: AppThreadGoalStatus) -> &'static str {
    match status {
        AppThreadGoalStatus::Active => "进行中",
        AppThreadGoalStatus::Paused => "已暂停",
        AppThreadGoalStatus::BudgetLimited => "受预算限制",
        AppThreadGoalStatus::Complete => "已完成",
    }
}

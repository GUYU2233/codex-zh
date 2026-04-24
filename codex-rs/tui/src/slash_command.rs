use strum::IntoEnumIterator;
use strum_macros::AsRefStr;
use strum_macros::EnumIter;
use strum_macros::EnumString;
use strum_macros::IntoStaticStr;

/// Commands that can be invoked by starting a message with a leading slash.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, EnumIter, AsRefStr, IntoStaticStr,
)]
#[strum(serialize_all = "kebab-case")]
pub enum SlashCommand {
    // DO NOT ALPHA-SORT! Enum order is presentation order in the popup, so
    // more frequently used commands should be listed first.
    Model,
    Fast,
    Approvals,
    Permissions,
    #[strum(serialize = "setup-default-sandbox")]
    ElevateSandbox,
    #[strum(serialize = "sandbox-add-read-dir")]
    SandboxReadRoot,
    Experimental,
    Memories,
    Skills,
    Review,
    Rename,
    New,
    Resume,
    Fork,
    Init,
    Compact,
    Plan,
    Collab,
    Agent,
    Side,
    // Undo,
    Copy,
    Diff,
    Mention,
    Status,
    DebugConfig,
    Title,
    Statusline,
    Theme,
    Mcp,
    Apps,
    Plugins,
    Logout,
    Quit,
    Exit,
    Feedback,
    Rollout,
    Ps,
    #[strum(to_string = "stop", serialize = "clean")]
    Stop,
    Clear,
    Personality,
    Realtime,
    Settings,
    TestApproval,
    #[strum(serialize = "subagents")]
    MultiAgents,
    // Debugging commands.
    #[strum(serialize = "debug-m-drop")]
    MemoryDrop,
    #[strum(serialize = "debug-m-update")]
    MemoryUpdate,
}

impl SlashCommand {
    /// User-visible description shown in the popup.
    pub fn description(self) -> &'static str {
        match self {
            SlashCommand::Feedback => "向维护者发送日志",
            SlashCommand::New => "在当前对话中开始一个新聊天",
            SlashCommand::Init => "创建包含 Codex 指令的 AGENTS.md 文件",
            SlashCommand::Compact => "总结对话，避免触及上下文上限",
            SlashCommand::Review => "审查我当前的改动并找出问题",
            SlashCommand::Rename => "重命名当前线程",
            SlashCommand::Resume => "恢复已保存的聊天",
            SlashCommand::Clear => "清空终端并开始新聊天",
            SlashCommand::Fork => "分叉当前聊天",
            // SlashCommand::Undo => "ask Codex to undo a turn",
            SlashCommand::Quit | SlashCommand::Exit => "退出 Codex",
            SlashCommand::Copy => "将上一条回复复制为 markdown",
            SlashCommand::Diff => "显示 git diff（包含未跟踪文件）",
            SlashCommand::Mention => "提及一个文件",
            SlashCommand::Skills => "使用技能改进 Codex 处理特定任务的方式",
            SlashCommand::Status => "显示当前会话配置和 token 用量",
            SlashCommand::DebugConfig => "显示配置层和需求来源用于调试",
            SlashCommand::Title => "配置终端标题中显示的项目",
            SlashCommand::Statusline => "配置状态行中显示的项目",
            SlashCommand::Theme => "选择语法高亮主题",
            SlashCommand::Ps => "列出后台终端",
            SlashCommand::Stop => "停止所有后台终端",
            SlashCommand::MemoryDrop => "DO NOT USE",
            SlashCommand::MemoryUpdate => "DO NOT USE",
            SlashCommand::Model => "选择要使用的模型和推理强度",
            SlashCommand::Fast => {
                "切换快速模式，以更高计划用量换取最快推理"
            }
            SlashCommand::Personality => "选择 Codex 的沟通风格",
            SlashCommand::Realtime => "切换实时语音模式（实验性）",
            SlashCommand::Settings => "配置实时麦克风/扬声器",
            SlashCommand::Plan => "切换到计划模式",
            SlashCommand::Collab => "更改协作模式（实验性）",
            SlashCommand::Agent | SlashCommand::MultiAgents => "切换当前 agent 线程",
            SlashCommand::Side => "在临时分叉中开始侧边对话",
            SlashCommand::Approvals => "选择允许 Codex 执行的操作",
            SlashCommand::Permissions => "选择允许 Codex 执行的操作",
            SlashCommand::ElevateSandbox => "设置提权 agent 沙箱",
            SlashCommand::SandboxReadRoot => {
                "允许沙箱读取目录：/sandbox-add-read-dir <absolute_path>"
            }
            SlashCommand::Experimental => "切换实验性功能",
            SlashCommand::Memories => "配置记忆的使用和生成",
            SlashCommand::Mcp => "列出已配置的 MCP 工具；使用 /mcp verbose 查看详情",
            SlashCommand::Apps => "管理应用",
            SlashCommand::Plugins => "浏览插件",
            SlashCommand::Logout => "退出 Codex 登录",
            SlashCommand::Rollout => "打印 rollout 文件路径",
            SlashCommand::TestApproval => "测试审批请求",
        }
    }

    /// Command string without the leading '/'. Provided for compatibility with
    /// existing code that expects a method named `command()`.
    pub fn command(self) -> &'static str {
        self.into()
    }

    /// Whether this command supports inline args (for example `/review ...`).
    pub fn supports_inline_args(self) -> bool {
        matches!(
            self,
            SlashCommand::Review
                | SlashCommand::Rename
                | SlashCommand::Plan
                | SlashCommand::Fast
                | SlashCommand::Mcp
                | SlashCommand::Side
                | SlashCommand::Resume
                | SlashCommand::SandboxReadRoot
        )
    }

    /// Whether this command remains available inside an active side conversation.
    pub fn available_in_side_conversation(self) -> bool {
        matches!(
            self,
            SlashCommand::Copy | SlashCommand::Diff | SlashCommand::Mention | SlashCommand::Status
        )
    }

    /// Whether this command can be run while a task is in progress.
    pub fn available_during_task(self) -> bool {
        match self {
            SlashCommand::New
            | SlashCommand::Resume
            | SlashCommand::Fork
            | SlashCommand::Init
            | SlashCommand::Compact
            // | SlashCommand::Undo
            | SlashCommand::Model
            | SlashCommand::Fast
            | SlashCommand::Personality
            | SlashCommand::Approvals
            | SlashCommand::Permissions
            | SlashCommand::ElevateSandbox
            | SlashCommand::SandboxReadRoot
            | SlashCommand::Experimental
            | SlashCommand::Memories
            | SlashCommand::Review
            | SlashCommand::Plan
            | SlashCommand::Clear
            | SlashCommand::Logout
            | SlashCommand::MemoryDrop
            | SlashCommand::MemoryUpdate => false,
            SlashCommand::Diff
            | SlashCommand::Copy
            | SlashCommand::Rename
            | SlashCommand::Mention
            | SlashCommand::Skills
            | SlashCommand::Status
            | SlashCommand::DebugConfig
            | SlashCommand::Ps
            | SlashCommand::Stop
            | SlashCommand::Mcp
            | SlashCommand::Apps
            | SlashCommand::Plugins
            | SlashCommand::Feedback
            | SlashCommand::Quit
            | SlashCommand::Exit
            | SlashCommand::Side => true,
            SlashCommand::Rollout => true,
            SlashCommand::TestApproval => true,
            SlashCommand::Realtime => true,
            SlashCommand::Settings => true,
            SlashCommand::Collab => true,
            SlashCommand::Agent | SlashCommand::MultiAgents => true,
            SlashCommand::Statusline => false,
            SlashCommand::Theme => false,
            SlashCommand::Title => false,
        }
    }

    fn is_visible(self) -> bool {
        match self {
            SlashCommand::SandboxReadRoot => cfg!(target_os = "windows"),
            SlashCommand::Copy => !cfg!(target_os = "android"),
            SlashCommand::Rollout | SlashCommand::TestApproval => cfg!(debug_assertions),
            _ => true,
        }
    }
}

/// Return all built-in commands in a Vec paired with their command string.
pub fn built_in_slash_commands() -> Vec<(&'static str, SlashCommand)> {
    SlashCommand::iter()
        .filter(|command| command.is_visible())
        .map(|c| (c.command(), c))
        .collect()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    use super::SlashCommand;

    #[test]
    fn stop_command_is_canonical_name() {
        assert_eq!(SlashCommand::Stop.command(), "stop");
    }

    #[test]
    fn clean_alias_parses_to_stop_command() {
        assert_eq!(SlashCommand::from_str("clean"), Ok(SlashCommand::Stop));
    }
}

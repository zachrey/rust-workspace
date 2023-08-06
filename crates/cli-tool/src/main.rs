mod cli;
// 使用 cli 模块中，具体的函数
use cli::instance::create_cli_instance;

#[tokio::main]
async fn main() {
    // 创建 cli 实例
    create_cli_instance();
}

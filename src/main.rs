use std::process::Command;

async fn kill_processes() {
    // Define the commands to be executed
    let commands = [
        "pkill ros2",
        "pkill rviz2",
        "pkill aggregator_node",
        r#"ps aux | grep python3 | grep ros2 | grep -v grep | awk '{ print "kill ", $2 }' | sh"#,
        r#"ps aux | grep python3 | grep rqt_reconfigure | grep -v grep | awk '{ print "kill ", $2 }' | sh"#,
        r#"ps aux | grep component_container | grep -v grep | awk '{ print "kill ", $2 }' | sh"#,
        r#"ps aux | grep robot_state_publisher | grep -v grep | awk '{ print "kill ", $2 }' | sh"#,
        r#"ps aux | grep topic_tools/relay | grep -v grep | awk '{ print "kill ", $2 }' | sh"#,
        r#"ps aux | grep "ros-args" | grep -v grep | awk '{ print "kill ", $2 }' | sh"#,
        "ros2 daemon stop",
        "ros2 daemon start",
    ];

    // Execute each command
    for cmd in commands.iter() {
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("Failed to execute command");

        if !output.status.success() {
            eprintln!("Command '{}' failed with status: {}", cmd, output.status);
        }
    }
}

#[tokio::main]
async fn main() {
    kill_processes().await;
}

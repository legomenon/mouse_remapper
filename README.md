# Mouse Remapper

The `mouse-remapper` is a utility that allows you to remap mouse events to specific keyboard actions using `evtest` and `evemu-event` on Linux.

## Notes

- This utility uses low-level interaction with input devices and may require administrative privileges to function properly.
- Be cautious when remapping input events, as it can have unintended consequences.
- Customize the code to suit your specific remapping needs and adjust time thresholds as necessary.

## Prerequisites

- Linux operating system
- Rust programming language
- `evtest` utility
- `evemu-event` utility

## Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/your-username/mouse-remapper.git
   ```

2. Build the project:

   ```sh
   cd mouse-remapper
   cargo build --release
   ```

## Usage

1. Ensure that you have the necessary privileges to access input devices. You may need to run the utility with elevated privileges (sudo).

2. Edit the `DEVICE_EVENT` constant in the `main.rs` file to match the event type you want to remap. Modify the constants and functions `execute`, `execute_up`, and `execute_down` to define your desired remapping behavior.

3. Run the `mouse-remapper` utility:

   ```sh
   cargo run --release
   ```

   The utility will monitor the specified input device using `evtest` and remap the events based on your configuration.


## Configuring sudoers File

To run the `mouse-remapper` utility without needing to enter the password each time, you can configure the `sudoers` file to allow the specific command to be executed with elevated privileges without a password prompt.

**Note: Modifying the `sudoers` file involves administrative access and can have security implications. Proceed with caution and ensure you understand the implications of these changes.**

1. Open a terminal and edit the sudoers file using the `visudo` command:

   ```sh
   sudo visudo
   ```

2. Add the following line to allow executing the `mouse-remapper` command without a password prompt:

   ```sh
   your-username ALL=(ALL) NOPASSWD: /path/to/mouse-remapper/target/release/mouse-remapper
   ```

   Replace `your-username` with your actual username and `/path/to/mouse-remapper/target/release/mouse-remapper` with the actual path to the compiled `mouse-remapper` executable.

3. Save and exit the `visudo` editor.

Now you should be able to run the `mouse-remapper` utility without needing to enter the password each time. Keep in mind that modifying the sudoers file can have security implications, so only grant privileges to specific commands that you trust.

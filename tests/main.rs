mod integration_tests {
    use std::io::{BufReader, BufWriter, Read, Write};
    use std::process::{Command, Stdio};

    #[derive(Debug)]
    struct CommandResult {
        stdout: String,
        stderr: String,
        exit_status: i32,
    }
    impl CommandResult {
        fn success(&self) -> bool {
            self.exit_status == 0
        }
    }

    fn run(image_file: &str, stdin: Option<&str>) -> CommandResult {
        let command = "./target/release/piet_programming_language";

        let args = vec![
            format!("./tests/{}", image_file),
            // "--verbose".to_string(),
        ];
        //impl {{{

        let mut child = Command::new(command);
        child.args(args);
        child.stdout(Stdio::piped()).stderr(Stdio::piped());
        if (stdin.is_some()) {
            child.stdin(Stdio::piped());
        }

        let mut child = match (child.spawn()) {
            Ok(c) => c,
            Err(e) => {
                return CommandResult {
                    stdout: String::new(),
                    stderr: e.to_string(),
                    exit_status: 1,
                }
            }
        };

        if (stdin.is_some()) {
            let mut stdin_ = BufWriter::new(child.stdin.take().unwrap());
            stdin_.write(stdin.unwrap().as_bytes()).unwrap();
            drop(stdin_);
        }

        let (stdout, stderr) = {
            let mut stdout = BufReader::new(child.stdout.take().unwrap());
            let mut stderr = BufReader::new(child.stderr.take().unwrap());

            let mut stdout_buf = String::new();
            stdout.read_to_string(&mut stdout_buf).unwrap();

            let mut stderr_buf = String::new();
            stderr.read_to_string(&mut stderr_buf).unwrap();
            (stdout_buf, stderr_buf)
        };

        let exit_status = child.wait().unwrap().code().unwrap();

        CommandResult {
            stdout,
            stderr,
            exit_status,
        }
        //}}}
    }

    #[test]
    fn test01() {
        let res = run("./test_images/hello_world.png", None);
        if (!res.success()) {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("Hello world!", res.stdout);
    }
}

//IMPORTANT NOTES:
// In this integration test, almost all of the samples exhibited in |https://www.dangermouse.net/esoteric/piet/samples.html| are tested.
// Some tests are set `ignored` because they result in infinite loops.
// As far as we investigated, we suspect the reason is not because out implementation is incorrect but because some samples are not standard-compliant (anymore).
// Especially, how white blocks shall be handled was not clarified in the first version of the spec, and it was afterwards clarified as seen in the latest spec.

mod integration_tests {
    use std::fs;
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
        __run(image_file, stdin, None)
    }

    fn __run(image_file: &str, stdin: Option<&str>, codel_size: Option<usize>) -> CommandResult {
        let command = "./target/release/piet_programming_language";
        if !fs::exists(command).unwrap() {
            panic!("Binary not found. Run `cargo build --release` first.");
        }

        let mut args = vec![
            format!("./tests/{}", image_file),
            // "--verbose".to_string(),
        ];
        if codel_size.is_some() {
            args.push("--codel-size".to_string());
            args.push(codel_size.unwrap().to_string());
        }
        //impl {{{

        let mut child = Command::new(command);
        child.args(args);
        child.stdout(Stdio::piped()).stderr(Stdio::piped());
        if stdin.is_some() {
            child.stdin(Stdio::piped());
        }

        let mut child = match child.spawn() {
            Ok(c) => c,
            Err(e) => {
                return CommandResult {
                    stdout: String::new(),
                    stderr: e.to_string(),
                    exit_status: 1,
                }
            }
        };

        if stdin.is_some() {
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
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("Hello world!", res.stdout);
    }

    //`ignore` because this results in an infinite loop.
    //The website says "Note: It has been reported that this program may be buggy; it goes into an infinite loop when traced with nPiet."
    #[test]
    #[ignore]
    fn test02() {
        let res = run("./test_images/fibonacci_numbers.gif", None);
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("Hello world!", res.stdout);
    }

    //`ignore` because the usage is not documented.
    #[test]
    #[ignore]
    fn test03() {
        let res = run("./test_images/towers_of_hanoi.gif", None);
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("Hello world!", res.stdout);
    }

    //`ignore` because non-standard colors are used.
    #[test]
    #[ignore]
    fn test04() {
        let res = run("./test_images/fancy_hello_world.png", None);
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("Hello world!", res.stdout);
    }

    //`ignore` because non-standard colors are used.
    #[test]
    #[ignore]
    fn test05() {
        let res = run("./test_images/prime_number_test.png", Some("1"));
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("Hello world!", res.stdout);
    }

    #[test]
    fn test06() {
        let res = run("./test_images/artistic_hello_world.png", None);
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("Hello, world!\n", res.stdout);
    }

    //`ignore` because this results in an infinite loop.
    #[test]
    #[ignore]
    fn test07() {
        let res = run("./test_images/artistic_hello_world_2.gif", None);
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("Hello, world!\n", res.stdout);
    }

    #[test]
    fn test08() {
        let res = run("./test_images/artistic_hello_world_3.gif", None);
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("Hello, world!\n", res.stdout);
    }

    #[test]
    fn test09() {
        let res = run("./test_images/artistic_hello_world_4.gif", None);
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("Hello, world!\n", res.stdout);
    }

    //`ignore` because this results in an infinite loop.
    #[test]
    #[ignore]
    fn test10() {
        let res = run("./test_images/piet.gif", None);
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("Hello, world!\n", res.stdout);
    }

    #[test]
    fn test11() {
        let res = run("./test_images/alpha.png", None);
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("abcdefghijklmnopqrstuvwxyz", res.stdout);
    }

    //`ignore` because non-standard colors are used.
    #[test]
    #[ignore]
    fn test12() {
        let res = run("./test_images/prime_number_generator.png", None);
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("abcdefghijklmnopqrstuvwxyz", res.stdout);
    }

    //`ignore` because it seems to work but some unnecessary characters are also output
    #[test]
    #[ignore]
    fn test13() {
        let res = run("./test_images/adder.png", Some("3 5"));
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("8", res.stdout);
    }

    #[test]
    fn test14() {
        let res = run("./test_images/pi.png", None);
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("31405\n\n", res.stdout);
    }

    //`ignore` because it doesn't work.
    #[test]
    #[ignore]
    fn test15() {
        let res = run("./test_images/euclid_algorithm.png", Some("10 4"));
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("2\n", res.stdout);

        let res = run("./test_images/euclid_algorithm.png", Some("17 19"));
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("1\n", res.stdout);
    }

    //`ignore` because this results in an infinite loop.
    #[test]
    #[ignore]
    fn test16() {
        let res = run("./test_images/japh.png", None);
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("31405\n\n", res.stdout);
    }

    #[test]
    fn test17() {
        let res = run("./test_images/power_function.png", Some("3 0"));
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("1\n", res.stdout);

        let res = run("./test_images/power_function.png", Some("3 4"));
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("81\n", res.stdout);
    }

    //`ignore` because non-standard colors are used.
    #[test]
    #[ignore]
    fn test18() {
        let res = run("./test_images/factorials.png", Some("0"));
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("1\n", res.stdout);

        let res = run("./test_images/power_function.png", Some("3"));
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("6\n", res.stdout);
    }

    //`ignore` because it seems to work but the output is very long.
    #[test]
    #[ignore]
    fn test19() {
        let res = run("./test_images/99_bottles_of_beer.png", None);
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("1\n", res.stdout);
    }

    #[test]
    fn test20() {
        let res = run("./test_images/mondrian_hello_world.png", None);
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("Hello, world!\n", res.stdout);
    }

    //`ignore` because it seems to work but some unnecessary characters are also output
    #[test]
    #[ignore]
    fn test21() {
        let res = run("./test_images/another_prime_tester.png", Some("0"));
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("0\nisnotprime", res.stdout);

        let res = run("./test_images/another_prime_tester.png", Some("1"));
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("1\nisnotprime", res.stdout);

        let res = run("./test_images/another_prime_tester.png", Some("2"));
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("2\nisprime", res.stdout);
    }

    //`ignore` because this results in an infinite loop.
    #[test]
    #[ignore]
    fn test22() {
        let res = run("./test_images/non_pastel_hello_world.png", None);
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("Hello, world!\n", res.stdout);
    }

    #[test]
    fn test23() {
        let res = run("./test_images/world_hello_world.png", None);
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("Hello, world!\n", res.stdout);
    }

    #[test]
    fn test24() {
        let res = run(
            "./test_images/day_of_week_calculator.png",
            Some("2023 3 29"),
        );
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("3\n", res.stdout);
    }

    //`ignore` because it results in an stack overflow
    #[test]
    #[ignore]
    fn test25() {
        let res = run("./test_images/assembled_piet_code.png", None);
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("3\n", res.stdout);
    }

    #[test]
    fn test26() {
        let res = run(
            "./test_images/brainfuck_interpreter.gif",
            Some(",+>,+>,+>,+.<.<.<.|sdhO"),
        );
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("Piet", res.stdout);
    }

    //`ignore` because the cow is printed but the read input is not printed.
    #[test]
    #[ignore]
    fn test27() {
        let res = run("./test_images/cowsay.png", Some("hello world"));
        if !res.success() {
            println!("{}", res.stderr);
        }
        println!("{}", res.stdout);
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("Piet", res.stdout);
    }

    //`ignore` because the usage is not documented.
    #[test]
    #[ignore]
    fn test28() {
        let res = run("./test_images/gnome_sort.png", Some("3 1 2 2"));
        if !res.success() {
            println!("{}", res.stderr);
        }
        println!("{}", res.stdout);
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("Piet", res.stdout);
    }

    #[test]
    fn test29() {
        let res = run("./test_images/tetris.png", None);
        if !res.success() {
            println!("{}", res.stderr);
        }
        println!("{}", res.stdout);
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("Tetris", res.stdout);
    }

    #[test]
    fn test30() {
        let res = run("./test_images/multi_codel_size.gif", None);
        if !res.success() {
            println!("{}", res.stderr);
        }
        println!("{}", res.stdout);
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("Piet\n", res.stdout);

        let res = __run("./test_images/multi_codel_size.gif", None, Some(4));
        if !res.success() {
            println!("{}", res.stderr);
        }
        println!("{}", res.stdout);
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("Hello world!\n", res.stdout);
    }

    //`ignore` because it seems to work but it's a highly interective program
    #[test]
    #[ignore]
    fn test31() {
        let res = run("./test_images/game_of_life.png", None);
        if !res.success() {
            println!("{}", res.stderr);
        }
        println!("{}", res.stdout);
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("Piet\n", res.stdout);
    }

    #[test]
    fn test32() {
        let res = run("./test_images/valentine_card.png", None);
        if !res.success() {
            println!("{}", res.stderr);
        }
        println!("{}", res.stdout);
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("I Love You Laura", res.stdout);
    }

    //`ignore` because this results in an infinite loop.
    #[test]
    #[ignore]
    fn test33() {
        let res = run("./test_images/quines_1.png", None);
        if !res.success() {
            println!("{}", res.stderr);
        }
        println!("{}", res.stdout);
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("I Love You Laura", res.stdout);
    }

    //`ignore` because this results in an infinite loop.
    #[test]
    #[ignore]
    fn test34() {
        let res = run("./test_images/quines_2.png", None);
        if !res.success() {
            println!("{}", res.stderr);
        }
        println!("{}", res.stdout);
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("I Love You Laura", res.stdout);
    }

    //`ignore` because it doesn't seem to work
    #[test]
    #[ignore]
    fn test35() {
        let res = run("./test_images/rock_paper_scissors.png", None);
        if !res.success() {
            println!("{}", res.stderr);
        }
        println!("{}", res.stdout);
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("I Love You Laura", res.stdout);
    }

    //`ignore` because non-standard colors are used.
    #[test]
    #[ignore]
    fn test36() {
        let res = run(
            "./test_images/more_piet_wall_art_in_cyan_magenta_and_blue.png",
            None,
        );
        if !res.success() {
            println!("{}", res.stderr);
        }
        println!("{}", res.stdout);
        assert!(res.success());
        assert!(res.stderr.is_empty());
        assert_eq!("I Love You Laura", res.stdout);
    }

    #[test]
    fn test37() {
        let res = run("./test_images/original___start_point_is_black.png", None);
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(!res.success());
        assert!(res.stdout.is_empty());
        assert!(res.stderr.contains("the top-left codel shall not be black"));
    }

    #[test]
    fn test38() {
        let res = run("./test_images/original___issue_02.png", None);
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert!(res.stdout.is_empty());
        assert!(res.stderr.is_empty());
    }

    #[test]
    fn test39() {
        let res = run("./test_images/original___issue_02_related.png", None);
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert_eq!("!", res.stdout);
        assert!(res.stderr.is_empty());
    }

    //ref: https://github.com/your-diary/piet_programming_language/issues/1#issuecomment-2407660388
    #[test]
    fn test40() {
        //ref: https://github.com/JanEricNitschke/TicTacToe/blob/main/tictactoe_piet/input1.txt
        let stdin = "0\n1\n2\n3\n4\n5\n6\n";

        let expected_stdout = r#"
---
---
---
Input:
X--
---
---
Input:
XO-
---
---
Input:
XOX
---
---
Input:
XOX
O--
---
Input:
XOX
OX-
---
Input:
XOX
OXO
---
Input:
Win for X!
XOX
OXO
X--
"#;

        let res = run("./test_images/tictactoe.png", Some(stdin));
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert_eq!(expected_stdout.trim_start(), res.stdout);
        assert!(res.stderr.is_empty());
    }

    //ref: https://github.com/your-diary/piet_programming_language/issues/1#issuecomment-2407660388
    //similar to `test40()` but with a different input
    #[test]
    fn test41() {
        //ref: https://github.com/JanEricNitschke/TicTacToe/blob/main/tictactoe_piet/input2.txt
        let stdin = "0\n4\n8\n1\n7\n6\n2\n5\n3\n";

        let expected_stdout = r#"
---
---
---
Input:
X--
---
---
Input:
X--
-O-
---
Input:
X--
-O-
--X
Input:
XO-
-O-
--X
Input:
XO-
-O-
-XX
Input:
XO-
-O-
OXX
Input:
XOX
-O-
OXX
Input:
XOX
-OO
OXX
Input:
Draw
XOX
XOO
OXX
"#;

        let res = run("./test_images/tictactoe.png", Some(stdin));
        if !res.success() {
            println!("{}", res.stderr);
        }
        assert!(res.success());
        assert_eq!(expected_stdout.trim_start(), res.stdout);
        assert!(res.stderr.is_empty());
    }
}

/*
Snakegame
John Bruhling @autotunafish

A snake game that uses the drawille crate to render images to the terminal.
Basically, he game spawns a thread that listens to keystrokes and writes them to a file.
While that happens, main thread does work and then reads that same file for any update to
the direction. The file acts as a (rx, tx) channel that doesn't block on the key read.
The keys still go to stdout but, w/ the frame rate set high enough, It is well hidden.

   Example coordinate diagram
  0,0____-_x_+_____5,0
    |  |  |  |  |  |
    -  |  |  |  |  |
    |  |  |  |  |  |
    y  |  |  |  |  |
    |  |  |  |  |  |
    +  |  |  |  |  |
  0,7__|__|__|__|__|5,7

*/
//TODO PAUSE BUTTON. JUST BIND ANOTHER KEY LIKE ESCAPE TO TRIGGER A BLOCKING EVENT MAYBE.
//ONLY PAUSE-UNPAUSE I THINK SHOULD SUFFICE.

//Drawille is the drawing tool.
extern crate drawille;
use drawille::Canvas;

//For the random placement of the food pieces.
use rand::Rng;

//For grabbing keystrokes.
use console::{Key, Term};

//Logo and Map files.
use logo::snakepic;
use maps::{mapb, mapc, mapd, mape, mapf, mapg, maph, mapi, mapj, mapk, mapl, mapm};

use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::process::exit;
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};

//Rodio is the Audio component.
use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};

mod logo;
mod maps;

//Draws a border.
fn border_100(mut c: Canvas, s: u32, p: u32) -> Canvas {
    c.line(1, 1, 1, p + 1);
    c.line(1, 1, s + 1, 1);
    c.line(1 + s, 1, s + 1, p + 1);
    c.line(1, p + 1, s + 1, p + 1);
    c
}

//Draws a snake box, take positional arg for top right corner and draw a 4x4 box and a deco line.
fn snake_box(mut c: Canvas, p: (u32, u32)) -> Canvas {
    c.line(p.0, p.1, p.0 + 3, p.1 + 0);
    c.line(p.0, p.1, p.0 + 0, p.1 + 3);
    c.line(p.0 + 3, p.1, p.0 + 3, p.1 + 3);
    c.line(p.0, p.1 + 3, p.0 + 3, p.1 + 3);
    //Decoration line
    c.line(p.0, p.1, p.0 + 1, p.1 + 1);
    c
}

//Draws an obstacle box, take positional arg for top right corner and draw a 4x4 box w diff deco.
fn obstacle_box(mut c: Canvas, p: (u32, u32)) -> Canvas {
    c.line(p.0, p.1, p.0 + 3, p.1 + 0);
    c.line(p.0, p.1, p.0 + 0, p.1 + 3);
    c.line(p.0 + 3, p.1, p.0 + 3, p.1 + 3);
    c.line(p.0, p.1 + 3, p.0 + 3, p.1 + 3);
    //Decoration line
    c.line(p.0 + 1, p.1 + 1, p.0 + 2, p.1 + 2);
    c
}

//Draw food box, take a random positional arg for top right corner and draw a 3x3 box
fn food_box(mut c: Canvas, p: (u32, u32)) -> Canvas {
    c.line(p.0, p.1, p.0 + 2, p.1 + 0);
    c.line(p.0, p.1, p.0 + 0, p.1 + 2);
    c.line(p.0 + 2, p.1, p.0 + 2, p.1 + 2);
    c.line(p.0, p.1 + 2, p.0 + 2, p.1 + 2);
    //Decoration line
    //c.line(p.0, p.1, p.0 + 1, p.1 + 1);
    c
}

//Collects the outside border coordinates for self, and obstacle, collision detection.
fn snake_skinner(p: (u32, u32)) -> Vec<(u32, u32)> {
    let points = vec![
        //top
        (p.0, p.1),
        (p.0 + 1, p.1),
        (p.0 + 2, p.1),
        (p.0 + 3, p.1),
        //bottom
        (p.0, p.1 + 3),
        (p.0 + 1, p.1 + 3),
        (p.0 + 2, p.1 + 3),
        (p.0 + 3, p.1 + 3),
        //left
        (p.0, p.1),
        (p.0, p.1 + 1),
        (p.0, p.1 + 2),
        (p.0, p.1 + 3),
        //right
        (p.0 + 3, p.1),
        (p.0 + 3, p.1 + 1),
        (p.0 + 3, p.1 + 2),
        (p.0 + 3, p.1 + 3),
    ];
    return points;
}

//Collects the outside border coordinates for collision detection of Food.
fn food_peeler(p: (u32, u32)) -> Vec<(u32, u32)> {
    let points = vec![
        //top
        (p.0, p.1),
        (p.0 + 1, p.1),
        (p.0 + 2, p.1),
        //bottom
        (p.0, p.1 + 2),
        (p.0 + 1, p.1 + 2),
        (p.0 + 2, p.1 + 2),
        //left
        (p.0, p.1),
        (p.0, p.1 + 1),
        (p.0, p.1 + 2),
        //right
        (p.0 + 2, p.1),
        (p.0 + 2, p.1 + 1),
        (p.0 + 2, p.1 + 2),
    ];
    return points;
}

/*
 ⣟⣹⣟⣹⣟⣹⣟⣹⣟⣹⣟⣹
⢸⣥⡇   MAIN
⢸⣥⡇
*/

fn main() {
    //Display the Opening Logo
    snakepic();
    sleep(Duration::from_millis(300));
    let mut confinput = String::new();
    io::stdin().read_line(&mut confinput).unwrap();

    //Used to eval a topscore, init eval is at beginning so must live out the game.
    let mut inttop = 0;

    'menu: loop {
        confinput.clear();

        //Read settings of Audio:Board_size:Map:Speed:Top_score. Write default values if none present.
        let setfile = "settings.txt".to_string();
        let mut settings = fs::read_to_string(&setfile).expect(
            "***No settings.txt file found!\n run 'touch settings.txt to correct the error ",
        );

        if settings.len() == 0 {
            let setstr = "A=0:B=1:M=A:S=3:T=0".to_string();
            let mut var_1 = File::create("settings.txt").expect("");
            var_1.write_all(&setstr.as_bytes()).expect("");
            settings = fs::read_to_string(&setfile).expect(
                "***No settings.txt file found!\n run 'touch settings.txt to correct the error ",
            );
        }

        //Get the settings values and assign to variables, create defaults.
        //Default size for board.
        let mut xdim = 100;
        let mut ydim = 100;

        //Default game wait time, controls overall snake speed.
        let mut speed = 35;

        //To print the speed level and top score.
        let mut spdref = String::new();

        let mut topscore = String::new();

        //For printing the board size value.
        let mut board_size = '1';

        //String of a possible new settings.
        let mut newset = String::new();

        //Holds the map selected.
        let mut map = 'A';

        //Holds the audio on/off value.
        let mut audio = 0;

        //For printing the audio on/off state.
        let mut sndref = String::new();

        //Pull out the 0th value until its len, stcount = 0.
        let mut stcount = settings.len();

        //Read settings in and set init quik-launch settings and write them back.
        'getsettings: loop {
            stcount -= 1;
            if stcount == 1 {
                break 'getsettings;
            }
            //THE FORMAT IS SIMPLE BUT NOT FLEXIBLE W/O FIRST ADJUSTING FOR IT.
            //CURRENTLY 0NLY HANDLES SINGLE CHAR OPTIONS BY DESIGN.

            let mut setter = settings.remove(0);

            //Audio
            match setter {
                'A' => {
                    settings.remove(0);
                    setter = settings.remove(0);
                    match setter {
                        '0' => {
                            audio = 0;
                            sndref = "OFF".to_string();
                            newset.push_str("A=0:");
                        }
                        '1' => {
                            audio = 1;
                            sndref = "ON".to_string();
                            newset.push_str("A=1:");
                        }

                        _ => (),
                    }
                }

                //Board size
                'B' => {
                    settings.remove(0);
                    setter = settings.remove(0);
                    board_size = setter;

                    //Match to the board.
                    match setter {
                        '1' => {
                            xdim = 100;
                            ydim = 100;
                            newset.push_str("B=1");
                        }
                        '2' => {
                            xdim = 100;
                            ydim = 140;
                            newset.push_str("B=2");
                        }
                        '3' => {
                            xdim = 100;
                            ydim = 180;
                            newset.push_str("B=3");
                        }
                        '4' => {
                            xdim = 160;
                            ydim = 100;
                            newset.push_str("B=4");
                        }
                        '5' => {
                            xdim = 160;
                            ydim = 140;
                            newset.push_str("B=5");
                        }
                        '6' => {
                            xdim = 160;
                            ydim = 180;
                            newset.push_str("B=6");
                        }
                        '7' => {
                            xdim = 220;
                            ydim = 100;
                            newset.push_str("B=7");
                        }
                        '8' => {
                            xdim = 220;
                            ydim = 140;
                            newset.push_str("B=8");
                        }
                        '9' => {
                            xdim = 220;
                            ydim = 180;
                            newset.push_str("B=9");
                        }
                        _ => (),
                    }
                }

                //Obstacle Map
                'M' => {
                    settings.remove(0);
                    setter = settings.remove(0);
                    map = setter;
                    let mut mapstr = ":M=".to_string();
                    mapstr.push(setter);
                    newset.push_str(&mapstr);
                }

                //Snake Speed
                'S' => {
                    settings.remove(0);
                    setter = settings.remove(0);
                    match setter {
                        '1' => {
                            speed = 50;
                            spdref = "1 Turtle".to_string();
                            newset.push_str(":S=1");
                        }
                        '2' => {
                            speed = 40;
                            spdref = "2 Slow".to_string();
                            newset.push_str(":S=2");
                        }
                        '3' => {
                            speed = 30;
                            spdref = "3 Normal".to_string();
                            newset.push_str(":S=3");
                        }
                        '4' => {
                            speed = 20;
                            spdref = "4 Fast".to_string();
                            newset.push_str(":S=4");
                        }
                        '5' => {
                            speed = 14;
                            spdref = "5 Rabbit".to_string();
                            newset.push_str(":S=5");
                        }
                        _ => (),
                    }
                }

                //Get topscore and eval against last game score. Overwrite if >=
                'T' => {
                    settings.remove(0);

                    topscore = settings.clone();

                    let popcheck = topscore.pop().unwrap();
                    if popcheck != '\n' {
                        topscore.push(popcheck);
                    }

                    let poptop: u32 = topscore.parse().unwrap();

                    if poptop <= inttop {
                        topscore = inttop.to_string();
                    }

                    newset.push_str(":T=");
                    newset.push_str(&topscore);

                    //Write it all to settings.txt
                    let mut var_1 = File::create("settings.txt").expect("");
                    var_1.write_all(&newset.as_bytes()).expect("");

                    break 'getsettings;
                }

                _ => (),
            }
        }

        //Begin main menu. User input for quik launch, setup, audio toggle, help, about, exit.
        println!("\n\n ****** Welcome to Snake! ******\n          TOP SCORE: {}\n\n > Press Enter to Launch Last Setup:", topscore);
        println!(
            "\n - Board: {} ({}x{})\n - Map: {}\n - Speed: {}\n - Sound: {}",
            &board_size, &xdim, &ydim, &map, &spdref, &sndref
        );

        println!(
            "\n\n > Enter S to Setup a New Game.\n\n > Enter A to Toggle Sound ON\\OFF.\n\n > Enter H for Help!\n\n > Enter X to EXIT."
        );

        io::stdin().read_line(&mut confinput).unwrap();

        confinput.pop();

        if confinput.len() == 0 {
            confinput.push('Q');
        }

        if confinput.len() >= 2 {
            println!(" ****** Invalid Input! Please Try Again. ******");
            continue;
        }

        confinput = confinput.to_ascii_uppercase();

        let mut nextset = String::new();

        match confinput.as_str() {
            "Q" => {
                // Launch Game setup.
            }
            "X" => {
                //Quit Game.
                println!("\n ****** Exiting NOW! Bye!! ******\n");
                break 'menu;
            }
            "H" => {
                println!("\n ****** Help! ******\n");
                println!(" Directions:\n Guide the snake to the food pieces to grow!");
                println!(" Avoid the snake body or game borders!\n\n");
                println!(" Start Menu: The player can quik-start by simply\n hitting the Enter key.\n The game will load the last settings used.\n\n The new game setup will prompt the player to\n select a\n > Board Size (9 settings) \n > Obstacle Course (13 settings)\n > Snake Speed (5 settings)\n\n Experiment with different game settings, as well\n as Font and Terminal Size.\n\n The functions that render are \"designed\" for\n MONO Spaced Fonts.\n\n This game was tested in Ubuntu Linux on a few\n different Terminal Emulators.\n Alacritty (Best), Terminology, Gnome Terminal,\n Xfce Terminal, Konsole and XTerm\n all performed excellently.\n Qterminal, seemingly, will not render correctly.\n");
                println!(" Movement:\n Up: UpArrow, W, K");
                println!(" Left: LeftArrow, A, H\n Right: RightArrow, D, L");
                println!(" Down: DownArrow, S, J \n\n");
                println!("Press Enter to Return to Main Menu");

                let mut helpinput = String::new();
                io::stdin().read_line(&mut helpinput).unwrap();
                continue;
            }

            "A" => {
                //Toggle audio on/off.
                let setfile = "settings.txt".to_string();
                let mut settings = fs::read_to_string(&setfile).expect(
            "***No settings.txt file found!\n run 'touch settings.txt to correct the error ",
        );

                settings.remove(0);
                settings.remove(0);
                let setter = settings.remove(0);
                match setter {
                    '0' => {
                        settings.insert_str(0, "A=1");
                    }
                    '1' => {
                        settings.insert_str(0, "A=0");
                    }
                    _ => (),
                }

                let mut var_1 = File::create("settings.txt").expect("");
                var_1.write_all(&settings.as_bytes()).expect("");
                continue;
            }

            "S" => {
                //Begin new game setup. Read audio option and then prompt for input.
                let setfile = "settings.txt".to_string();
                let mut settings = fs::read_to_string(&setfile).expect(
            "***No settings.txt file found!\n run 'touch settings.txt to correct the error ",
        );

                settings.remove(0);
                settings.remove(0);
                let setter = settings.remove(0);
                match setter {
                    '1' => {
                        nextset.insert_str(0, "A=1");
                    }
                    '0' => {
                        nextset.insert_str(0, "A=0");
                    }
                    _ => (),
                }

                //Choose Board Size.
                'bd: loop {
                    println!("\n\n ****** Choose Board size ******\n\n > Enter 1-9 to select (default: 1):\n\n");
                    println!(" (horixontal)x(vertical)\n\n 1: 100x100    2: 100x140    3: 100x180\n\n 4: 160x100    5: 160x140    6: 160x180\n\n 7: 220x100    8: 220x140    9: 220x180\n\n Tip: Resize your Font and Terminal\n to fit the board and bottom line.\n\n");

                    confinput.clear();
                    io::stdin().read_line(&mut confinput).unwrap();

                    confinput.pop();
                    if confinput.len() == 0 {
                        confinput.push('1');
                    }

                    if confinput.len() >= 2 {
                        println!(" ****** Invalid Input! Please Try Again. ******");
                        continue;
                    }

                    confinput = confinput.to_ascii_uppercase();
                    match confinput.as_str() {
                        "1" => {
                            xdim = 100;
                            ydim = 100;
                            nextset.push_str("B=1")
                        }
                        "2" => {
                            xdim = 100;
                            ydim = 140;
                            nextset.push_str("B=2")
                        }
                        "3" => {
                            xdim = 100;
                            ydim = 180;
                            nextset.push_str("B=3")
                        }
                        "4" => {
                            xdim = 160;
                            ydim = 100;
                            nextset.push_str("B=4")
                        }
                        "5" => {
                            xdim = 160;
                            ydim = 140;
                            nextset.push_str("B=5")
                        }
                        "6" => {
                            xdim = 160;
                            ydim = 180;
                            nextset.push_str("B=6")
                        }
                        "7" => {
                            xdim = 220;
                            ydim = 100;
                            nextset.push_str("B=7")
                        }
                        "8" => {
                            xdim = 220;
                            ydim = 140;
                            nextset.push_str("B=8")
                        }
                        "9" => {
                            xdim = 220;
                            ydim = 180;
                            nextset.push_str("B=9")
                        }

                        _ => {
                            println!(" ****** Invalid Input! Please Try Again. ******");
                            continue;
                        }
                    }

                    break 'bd;
                }

                println!(" You selected Board: {}: X{} Y{}", &confinput, &xdim, &ydim);

                //Choose Obstacle Map. Currently only A-M.
                'map: loop {
                    println!(" \n\n ******* Select Map **********\n\n Enter A-N to Select\n\n A: No Obstacle (Default)\n B: Center Horizontal Line\n C: Center Vertical Line\n D: Center Cross\n E: Offset Double Horiz Lines \n F: Parallel Double Vertical Lines\n G: Horizontal (Sideways) T\n H: Vertical T (Seperated Top)\n I: U Shape\n J: Solid Box\n K: < > but Straightened\n L: Zig-Zag but Straightened\n M: Double Horiz Solid Box");

                    confinput.clear();
                    io::stdin().read_line(&mut confinput).unwrap();

                    confinput.pop();
                    if confinput.len() == 0 {
                        confinput.push('A');
                    }

                    if confinput.len() >= 2 {
                        println!(" ****** Invalid Input! Please Try Again. ******");
                        continue;
                    }

                    confinput = confinput.to_ascii_uppercase();
                    let alphachek = confinput.pop().unwrap();

                    if alphachek.is_alphabetic() == false {
                        println!(" ****** Invalid Input! Please Try Again. ******");
                        continue;
                    }

                    match alphachek {
                        'N' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'U' | 'V' | 'W' | 'X' | 'Y' | 'Z' => {
                            println!(" ****** Invalid Input! Please Try Again. ******");
                            continue;
                        }
                        _ => (),
                    }

                    map = alphachek;
                    let mut mapstr = ":M=".to_string();
                    mapstr.push(alphachek);
                    nextset.push_str(&mapstr);

                    break 'map;
                }

                println!(" You selected Map: {}", map);

                //Choose Snake Speed.
                'sp: loop {
                    println!("\n\n ****** Choose Snake Speed ******\n\n Enter 1-5 to select (Default: 3)\n\n");

                    println!("1: Turtle\n2: Slow\n3: Normal\n4: Fast\n5: Rabbit\n\n");

                    confinput.clear();

                    io::stdin().read_line(&mut confinput).unwrap();

                    confinput.pop();
                    if confinput.len() == 0 {
                        confinput.push('3');
                    }

                    if confinput.len() >= 2 {
                        println!(" ****** Invalid Input! Please Try Again. ******");
                        continue;
                    }

                    confinput = confinput.to_ascii_uppercase();
                    match confinput.as_str() {
                        "1" => {
                            speed = 50;
                            nextset.push_str(":S=1")
                        }
                        "2" => {
                            speed = 40;
                            nextset.push_str(":S=2")
                        }
                        "3" => {
                            speed = 30;
                            nextset.push_str(":S=3")
                        }
                        "4" => {
                            speed = 20;
                            nextset.push_str(":S=4")
                        }
                        "5" => {
                            speed = 14;
                            nextset.push_str(":S=5")
                        }

                        _ => {
                            println!(" ****** Invalid Input! Please Try Again. ******");
                            continue;
                        }
                    }

                    break 'sp;
                }

                nextset.push_str(":T=");
                nextset.push_str(&topscore);

                //Write it all back to the settings file.
                let mut var_1 = File::create("settings.txt").expect("");
                var_1.write_all(&nextset.as_bytes()).expect("");

                //Queue begin game.
                println!(" You selected: {}\n\n > Press Enter to Begin", &confinput);

                let mut confinput = String::new();
                io::stdin().read_line(&mut confinput).unwrap();
            }

            _ => {
                println!(" ****** Invalid Input! Please Try Again. ******");
                continue;
            }
        }

        //Set up the in-game variables.
        //The size values match the borders or they will collide.
        let mut canvas = Canvas::new(xdim, ydim);

        //The Snake body.
        //(x, y), 625 is theoretical max for 98x98, set x4 should be overkill but ok for Mondo sized boards.
        //We only ever go over the populated snake.
        //Directions == 1 | 2 | 3 | 4 -NSWE-UDLR,
        let mut body = [(0, 0); 2500];

        //Record of the moves for the boxes at each tick, excess is trimmed below.
        //THE MAIN PURPOSE OF THIS VEC WAS MADE OBSOLETE DURING DEV BUT IS STILL PARTIALLY NECESSARY.
        //WHETHER IT CAN BE OPTIMIZED SMALLER SEEMS POSSIBLE BUT ITS NEVER VERY MUCH MEMORY, IDK.
        let mut movequeue = vec![4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4];

        //Records the border of the snake.
        let mut snakeskin = vec![(0, 0)];

        //The points on the front of the head wrt to direction.
        let mut snakeface = vec![(0, 0)];

        //This is a pass through variable so needs a '_'.
        let mut _skinchips = vec![(0, 0)];

        //Indicates a collision has occured, written to a file and initiates thread kill.
        let mut gameover = "0";

        //Total number of boxes, increments w/ food.
        let mut totallength = 4;

        //Holds the points of the Obstacle boxes.
        let mut obstacle_body = vec![(0, 0)];

        //THIS SETS THE START POSITION BUT IS CURRENTLY NOT USED TO SET A NEW POSITION
        //startx HAS TO BE AT LEAST 30, Y AT LEAST 2
        let startx = 30;
        let starty = 10;

        //Match on the selected value, pass the xdim ydim to get the scaled coordinates.
        //See maps.rs for details, would be a LOT here!
        match map {
            //A = NO-Obstacles
            'A' => {
                obstacle_body.remove(0);
            }
            'B' => {
                obstacle_body = mapb(xdim, ydim);
                obstacle_body.remove(0);
            }
            'C' => {
                obstacle_body = mapc(xdim, ydim);
                obstacle_body.remove(0);
            }
            'D' => {
                obstacle_body = mapd(xdim, ydim);
                obstacle_body.remove(0);
            }
            'E' => {
                obstacle_body = mape(xdim, ydim);
                obstacle_body.remove(0);
            }
            'F' => {
                obstacle_body = mapf(xdim, ydim);
                obstacle_body.remove(0);
            }
            'G' => {
                obstacle_body = mapg(xdim, ydim);
                obstacle_body.remove(0);
            }
            'H' => {
                obstacle_body = maph(xdim, ydim);
                obstacle_body.remove(0);
            }
            'I' => {
                obstacle_body = mapi(xdim, ydim);
                obstacle_body.remove(0);
            }
            'J' => {
                obstacle_body = mapj(xdim, ydim);
                obstacle_body.remove(0);
            }
            'K' => {
                obstacle_body = mapk(xdim, ydim);
                obstacle_body.remove(0);
            }
            'L' => {
                obstacle_body = mapl(xdim, ydim);
                obstacle_body.remove(0);
            }
            'M' => {
                obstacle_body = mapm(xdim, ydim);
                obstacle_body.remove(0);
            }
            /*
             * RESERVED FOR FUTURE MAPS
            'N' => {}
            'O' => {}
            'P' => {}
            'Q' => {}
            'R' => {}
            'S' => {}
            'T' => {}
            'U' => {}
            'V' => {}
            'W' => {}
            'X' => {}
            'Y' => {}
            'Z' => {}
            */
            _ => {
                obstacle_body.remove(0);
            }
        }

        //Build the obstacle_skin.
        //Holds all the outside points of the obstacles. Calculated once since stationary.
        let mut obstacle_skin = vec![(0, 0)];

        let mut _obschips = vec![(0, 0)];

        let mut count = 0;
        'obstacle: loop {
            if count == obstacle_body.len() {
                break 'obstacle;
            }

            let position = obstacle_body[count];

            //Collect the outside points of the obstacle for collision detection.
            _obschips = snake_skinner(position);
            for i in _obschips {
                obstacle_skin.push(i);
            }
            count += 1;
        }

        //Records all of the points the head has moved through, equals * 6 the total number of boxes to set the
        //trailing box position (because of an intermittent bug the would skew the trailing box positions)
        //as well as record the positions for new added boxes.
        //neckrecord ASSUMES START MOVING RIGHT, HAVE TO CHANGE starty TO MAKE VERTICAL START.
        let mut neckrecord = vec![
            (startx, starty),
            (startx - 1, starty),
            (startx - 2, starty),
            (startx - 3, starty),
            (startx - 4, starty),
            (startx - 5, starty),
            (startx - 6, starty),
            (startx - 7, starty),
            (startx - 8, starty),
            (startx - 9, starty),
            (startx - 10, starty),
            (startx - 11, starty),
            (startx - 12, starty),
            (startx - 13, starty),
            (startx - 14, starty),
            (startx - 15, starty),
            (startx - 16, starty),
            (startx - 17, starty),
            (startx - 18, starty),
            (startx - 19, starty),
            (startx - 20, starty),
            (startx - 21, starty),
            (startx - 22, starty),
            (startx - 23, starty),
            (startx - 24, starty),
            (startx - 25, starty),
            (startx - 26, starty),
            (startx - 27, starty),
            (startx - 28, starty),
            (startx - 29, starty),
        ];

        //Populate initial snake coordinates.
        body[0] = (startx, starty);
        body[1] = (startx - 4, starty);
        body[2] = (startx - 8, starty);
        body[3] = (startx - 12, starty);

        //Sets a value to generate a food square.
        let mut isfood = 1;

        //Initial food spot.
        let mut foodspot = (startx + 35, starty);

        //Holds the outside points of the food box.
        let mut _foodwrapper = vec![(0, 0)];

        //Boxes eaten. Yum.
        let mut score = 0;

        //Directional key variable from user input. Snake starts going right -->
        let mut testmove = String::from("4");

        //Write first direction value if not present.
        let mut var_1 = File::create("moves.txt").expect("");
        var_1.write_all(&testmove.as_bytes()).expect("");

        //Write game-on value if not present.
        let mut var_1 = File::create("onoff.txt").expect("");
        var_1.write_all(&gameover.as_bytes()).expect("");

        //Spawn a thread to listen on keystrokes, write to a file, check below.
        //This normally blocks so it goes in a spawned thread.
        let keybind = thread::spawn(move || {
            #[allow(unused_labels)]
            'punch: loop {
                let term = Term::stdout();
                let key = term.read_key().unwrap();
                match key {
                    Key::ArrowLeft
                    | Key::Char('h')
                    | Key::Char('a')
                    | Key::Char('H')
                    | Key::Char('A') => {
                        testmove = "3".to_string();
                    }
                    Key::ArrowRight
                    | Key::Char('l')
                    | Key::Char('d')
                    | Key::Char('L')
                    | Key::Char('D') => {
                        testmove = "4".to_string();
                    }
                    Key::ArrowUp
                    | Key::Char('k')
                    | Key::Char('w')
                    | Key::Char('K')
                    | Key::Char('W') => {
                        testmove = "1".to_string();
                    }
                    Key::ArrowDown
                    | Key::Char('j')
                    | Key::Char('s')
                    | Key::Char('J')
                    | Key::Char('S') => {
                        testmove = "2".to_string();
                    }
                    _ => (),
                };

                //Write the new keydown captured to a file.
                let mut var_1 = File::create("moves.txt").expect("");
                var_1.write_all(&testmove.as_bytes()).expect("");
                sleep(Duration::new(0, 16000000));

                //Check if the game is over and end this thread.
                //This thread joins at the very end of main and blocks on the final key read.
                let tokfile = String::from("onoff.txt");
                let schroedinger = fs::read_to_string(&tokfile).expect(
                    "***No onoff.txt file found!\n run 'touch onoff.txt to correct the error ",
                );

                //The gameover value is LOW until gameover sets HIGH.
                if schroedinger == "1" {
                    sleep(Duration::from_millis(300));

                    break 'punch;
                }
            }
        });

        //Spawn a thread to make sounds.
        //THERE USED TO BE MORE FUNCTIONALITY HERE THAT MADE A BEEPBEEPBEEPBEP BUT WAS ANNOYING
        let sounder = thread::spawn(move || {
            //Read in the toggled sound option on/off.
            let setfile = "settings.txt".to_string();
            let mut settings = fs::read_to_string(&setfile).expect(
                "***No settings.txt file found!\n run 'touch settings.txt to correct the error ",
            );

            settings.remove(0);
            settings.remove(0);
            let setter = settings.remove(0);

            //Build an audio strem object.
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();

            #[allow(unused_labels)]
            'beep: loop {
                //If audio is on
                if setter == '1' {
                    //Check for a beep indicator.
                    let tikfile = String::from("beep.txt");
                    let chime = fs::read_to_string(&tikfile).expect(
                        "***No beep.txt file found!\n run 'touch beep.txt' to correct the error ",
                    );

                    //Match on the beep (there was another, hence the match, but it wasnt good).
                    match chime.as_str() {
                        "2" => {
                            let pingeat = SineWave::new(290.0)
                                .take_duration(Duration::from_secs_f32(0.10))
                                .amplify(0.07);
                            sink.append(pingeat);
                            //This 2nd sound at zero volume smooths out the above tone exit.
                            let pingeat = SineWave::new(290.0)
                                .take_duration(Duration::from_secs_f32(0.05))
                                .amplify(0.0);
                            sink.append(pingeat);
                            sink.sleep_until_end();
                            //Write the nuetral sound indicator to a file.
                            let nuetralsound = "1";
                            let mut var_1 = File::create("beep.txt").expect("");
                            var_1.write_all(&nuetralsound.as_bytes()).expect("");
                        }

                        _ => (),
                    }
                }

                //Check if the game is over and end this thread.
                //This thread joins at the very end of main and blocks on the final key read.
                let tokfile = String::from("onoff.txt");
                let schroedinger = fs::read_to_string(&tokfile).expect(
                    "***No onoff.txt file found!\n run 'touch onoff.txt to correct the error ",
                );
                //println!("{:?}", schroedinger);

                //The gameover value is LOW until gameover sets HIGH.
                if schroedinger == "1" {
                    sleep(Duration::from_millis(300));

                    break 'beep;
                }
                sleep(Duration::new(0, 32000000));
            }
        });

        /*
          ⡶⢺⣋⣿⣋⣿⣋⣿⣋⣿⣋⡇
          ⡿⢻⡤⡄     GAME LOOP                                   ⣀
        ⣀⣀⣀⣉⣙⣺⣋⣇⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀
              */

        #[allow(unused_labels)]
        'game: loop {

            //Clear the Terminal screen and set cursor position.
            //Sets Lower Left. It doesn't look bad.
            //print!("{}[2J", 27 as char);
            //Sets Upper Left.
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
           
            //Time of the new tick.
            let start = Instant::now();

            //Initial canvas.
            canvas = border_100(canvas, xdim, ydim);

            //Draw the obstacle boxes.
            let mut count = 0;
            loop {
                if count == obstacle_body.len() {
                    break;
                }
                canvas = obstacle_box(canvas, obstacle_body[count]);
                count += 1;
            }

            //Clear stale data.
            snakeskin.clear();
            snakeface.clear();

            //Trim excessive unnecessary values.
            if movequeue.len() >= totallength * 6 {
                movequeue.pop();
            }

            if neckrecord.len() >= totallength * 6 {
                neckrecord.pop();
            }

            //Get the snake positions.
            let mut count = 0;
            'snake: loop {
                let position = body[count];
                let skinposition = position.clone();

                //This indicates the end of the snake data (0,0).
                if position.0 == 0 && position.1 == 0 {
                    break 'snake;
                }

                //Draw the snake.
                canvas = snake_box(canvas, position);

                //Collect the outside points of the snake for collision detection.
                _skinchips = snake_skinner(skinposition);
                for i in _skinchips {
                    snakeskin.push(i);
                }
                count += 1;
            }

            //Ckeck if the food was eaten or not. If yes,
            if isfood == 0 {
                //Generate two random values within range and validate.
                'food: loop {
                    let mut x = rand::thread_rng();
                    let mut y = rand::thread_rng();

                    //THE VALUES 2 AND -2 STILL PLACE THE FOOD ON THE BORDER EDGE WHICH UPS THE DIFF, GOOD.
                    let xrng = x.gen_range(2..=xdim - 2);
                    let yrng = y.gen_range(2..=ydim - 2);

                    //Pass to foodspot.
                    foodspot = (xrng, yrng);

                    //Get the outside points of the food box to compare to occupied space.
                    //If points collide, recalculate.
                    let foodskin = food_peeler(foodspot);

                    for i in &snakeskin {
                        for j in &foodskin {
                            if i == j {
                                continue 'food;
                            }
                        }
                    }

                    for i in &obstacle_skin {
                        for j in &foodskin {
                            if i == j {
                                continue 'food;
                            }
                        }
                    }

                    break 'food;
                }

                //Set isfood HIGH now.
                isfood = 1;
                //Next Nom Nom collision sets it LOW.

                //Set the new snake box coordinates inside body at the ass-end.
                body[totallength] = neckrecord[totallength * 4];

                //Plus one snake boxes and score.
                totallength += 1;
                score += 1;
            }

            //Draw food box.
            canvas = food_box(canvas, foodspot);

            //Get outside verified points of food box.
            _foodwrapper = food_peeler(foodspot);

            //Update the snake body positions, head first, we need to compare the next and last directions for validity.
            //Cannot reverse onto own body.
            let new_direction = movequeue[0];
            let mut new_heading = body[0];
            testmove = new_direction.to_string();

            //Get the face points for collision detection from the head points wrt to direction.
            match new_direction {
                1 => {
                    snakeface.insert(0, snakeskin[0]);
                    snakeface.insert(1, snakeskin[1]);
                    snakeface.insert(2, snakeskin[2]);
                    snakeface.insert(3, snakeskin[3]);
                }
                2 => {
                    snakeface.insert(0, snakeskin[4]);
                    snakeface.insert(1, snakeskin[5]);
                    snakeface.insert(2, snakeskin[6]);
                    snakeface.insert(3, snakeskin[7]);
                }
                3 => {
                    snakeface.insert(0, snakeskin[8]);
                    snakeface.insert(1, snakeskin[9]);
                    snakeface.insert(2, snakeskin[10]);
                    snakeface.insert(3, snakeskin[11]);
                }
                4 => {
                    snakeface.insert(0, snakeskin[12]);
                    snakeface.insert(1, snakeskin[13]);
                    snakeface.insert(2, snakeskin[14]);
                    snakeface.insert(3, snakeskin[15]);
                }

                _ => (),
            }

            //Check for a collision of the snake body and the borders.
            //Remove the head and iter over the rest. Compare decapitated body to face.
            let headless = snakeskin.split_off(16);

            //If body collision, or
            for i in &snakeface {
                for j in &headless {
                    if i == j {
                        gameover = "1";
                    }
                }
            }

            //if border or obstacle collision, set gameover HIGH.
            for i in &snakeface {
                if i.0 == 1 {
                    gameover = "1";
                } else if i.0 == xdim + 1 {
                    gameover = "1";
                }
                if i.1 == 1 {
                    gameover = "1";
                } else if i.1 == ydim + 1 {
                    gameover = "1";
                }
            }

            for i in &snakeface {
                for j in &obstacle_skin {
                    if i == j {
                        gameover = "1";
                    }
                }
            }

            //Lunch! Food collision sets isfood LOW and sets the beep trigger.
            for i in &snakeface {
                for j in &_foodwrapper {
                    if i == j {
                        inttop += 1;
                        isfood = 0;
                        let eatsound = "2";
                        let mut var_1 = File::create("beep.txt").expect("");
                        var_1.write_all(&eatsound.as_bytes()).expect("");
                    }
                }
            }

            //Read in a possible new move, compare and validate against reversal.
            let mut possiblemove = fs::read_to_string("moves.txt")
                .expect("***No moves.txt file found!\n run 'touch moves.txt to correct the error ");
            if possiblemove != testmove {
                let mut opposite = String::new();
                match possiblemove.as_str() {
                    "1" => {
                        opposite.push_str("2");
                    }
                    "2" => {
                        opposite.push_str("1");
                    }
                    "3" => {
                        opposite.push_str("4");
                    }
                    "4" => {
                        opposite.push_str("3");
                    }
                    _ => (),
                }
                //If the input direction does reverse, use last good direction.
                let revcheck = movequeue[0..4].to_vec();
                for i in revcheck {
                    if i.to_string() == opposite {
                        possiblemove = testmove.clone();
                    }
                }

                //Match the new validated move to a new coordinate.
                match possiblemove.as_str() {
                    "1" => {
                        new_heading.1 -= 1;
                        movequeue.insert(0, 1);
                    }
                    "2" => {
                        new_heading.1 += 1;
                        movequeue.insert(0, 2);
                    }
                    "3" => {
                        new_heading.0 -= 1;
                        movequeue.insert(0, 3);
                    }
                    "4" => {
                        new_heading.0 += 1;
                        movequeue.insert(0, 4);
                    }
                    _ => (),
                }
                body[0] = new_heading;
            } else {
                // Or, match the last good move to a new coordinate.
                match testmove.as_str() {
                    "1" => {
                        new_heading.1 -= 1;
                        movequeue.insert(0, 1);
                    }
                    "2" => {
                        new_heading.1 += 1;
                        movequeue.insert(0, 2);
                    }
                    "3" => {
                        new_heading.0 -= 1;
                        movequeue.insert(0, 3);
                    }
                    "4" => {
                        new_heading.0 += 1;
                        movequeue.insert(0, 4);
                    }
                    _ => (),
                }
                body[0] = new_heading;
            }

            //Insert latest coordinate into the neckrecord.
            neckrecord.insert(0, new_heading);

            //Update the position of all the snakeboxes.
            let mut tailcount = 0;
            loop {
                if tailcount == totallength {
                    break;
                }
                body[tailcount] = neckrecord[(tailcount) * 4];
                tailcount += 1;
            }

            //inttop is saved to be evaluated against the previous topscore.
            inttop = score;

            //Prints the phrase Game Over.
            let mut endgame = String::new();
            if gameover == "1" {
                endgame = "***!GAME OVER!***!PRESS ANY KEY!***".to_string();
            }

            //Start the sub-ticker, controls the framerate.
            let mut shorttime = start.elapsed().as_millis();

            println!(
                "\n{}\n SCORE: {}  {}",
                canvas.clone().frame(),
                score,
                endgame
            );
            io::stdout().flush().unwrap();

            //Sleep, wake and check, and repeat until the speed time has elapsed. Then proceed.
            'ticker: loop {
                if shorttime >= speed {
                    break 'ticker;
                }

                //This controls the sleep period as nanos.
                sleep(Duration::new(0, 1600000));

                //Update the time elapsed for ticker shorttime.
                shorttime = start.elapsed().as_millis();
                continue;
            }

            //Erase old drawing.
            canvas.clear();

            //If gameover is HIGH, break game loop.
            if gameover == "1" {
                break 'game;
            }
        }

        //Write a "1" to onoff file to be read in the spawned thread so that it may exit from within itself.
        let mut var_1 = File::create("onoff.txt").expect("");
        var_1.write_all(&gameover.as_bytes()).expect("");

        //Play the crash sound if the audio is on.
        if audio == 1 {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();

            let ping1 = SineWave::new(140.0)
                .take_duration(Duration::from_secs_f32(0.50))
                .amplify(0.37);
            sink.append(ping1);

            //This 2nd sound at zero volume eliminates any pops that can happen to the above tone.
            let ping1 = SineWave::new(140.0)
                .take_duration(Duration::from_secs_f32(0.05))
                .amplify(0.0);
            sink.append(ping1);

            sink.sleep_until_end();
            sleep(Duration::from_millis(100));
        }
        //Join both the threads so that they can both exit properly.
        keybind.join().unwrap();
        sounder.join().unwrap();
        sleep(Duration::from_millis(200));
    }

    exit(0);
}

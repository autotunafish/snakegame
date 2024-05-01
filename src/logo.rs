/*
 PRINTS THE SNAKE LOGO SCREEN.
 DESIGNED FOR MONO SPACED FONTS
 */

extern crate drawille;
use drawille::Canvas;

pub fn snakepic() {
    let mut canvas = Canvas::new(100, 100);
    let _size1 = 100;
    let selsize = _size1;
    //3
    canvas.line(30, 3, 31, 3);
    //4
    canvas.line(29, 4, 36, 4);
    //5
    canvas.line(28, 5, 28, 5);
    canvas.line(37, 5, 39, 5);
    //6
    canvas.line(27, 6, 27, 6);
    canvas.line(39, 6, 40, 6);
    //7
    canvas.line(26, 7, 26, 7);
    canvas.line(28, 7, 28, 7);
    canvas.line(40, 7, 41, 7);
    //8
    canvas.line(26, 8, 26, 8);
    canvas.line(28, 8, 28, 8);
    canvas.line(37, 8, 37, 8);
    canvas.line(41, 8, 42, 8);
    //9
    canvas.line(25, 9, 25, 9);
    canvas.line(28, 9, 29, 9);
    canvas.line(36, 9, 37, 9);
    canvas.line(42, 9, 43, 9);
    //10
    canvas.line(24, 10, 24, 10);
    canvas.line(28, 10, 29, 10);
    canvas.line(34, 10, 36, 10);
    canvas.line(43, 10, 44, 10);
    //11
    canvas.line(23, 11, 24, 11);
    canvas.line(34, 11, 36, 11);
    canvas.line(44, 11, 58, 11);
    //12
    canvas.line(23, 12, 24, 12);
    canvas.line(45, 12, 46, 12);
    canvas.line(58, 12, 59, 12);
    //13
    canvas.line(23, 13, 24, 13);
    canvas.line(46, 13, 65, 13);
    //14
    canvas.line(22, 14, 26, 14);
    canvas.line(65, 14, 66, 14);
    //15
    canvas.line(21, 15, 22, 15);
    canvas.line(24, 15, 28, 15);
    canvas.line(66, 15, 66, 15);
    //16
    canvas.line(20, 16, 28, 16);
    canvas.line(31, 16, 31, 16);
    canvas.line(64, 16, 67, 16);
    //17
    canvas.line(20, 17, 27, 17);
    canvas.line(29, 17, 31, 17);
    canvas.line(67, 17, 68, 17);
    //18
    canvas.line(19, 18, 26, 18);
    canvas.line(30, 18, 34, 18);
    canvas.line(36, 18, 37, 18);
    canvas.line(58, 18, 63, 18);
    canvas.line(68, 18, 69, 18);
    //19
    canvas.line(19, 19, 19, 19);
    canvas.line(21, 19, 24, 19);
    canvas.line(30, 19, 39, 19);
    canvas.line(57, 19, 64, 19);
    canvas.line(70, 19, 70, 19);
    //20
    canvas.line(18, 20, 22, 20);
    canvas.line(30, 20, 40, 20);
    canvas.line(57, 20, 64, 20);
    canvas.line(70, 20, 71, 20);
    //21
    canvas.line(18, 21, 21, 21);
    canvas.line(29, 21, 36, 21);
    canvas.line(39, 21, 44, 21);
    canvas.line(57, 21, 65, 21);
    canvas.line(71, 21, 71, 21);
    //22
    canvas.line(17, 22, 20, 22);
    canvas.line(29, 22, 34, 22);
    canvas.line(43, 22, 43, 22);
    canvas.line(45, 22, 45, 22);
    canvas.line(39, 22, 41, 22);
    canvas.line(58, 22, 65, 22);
    canvas.line(71, 22, 72, 22);
    //23
    canvas.line(17, 23, 17, 23);
    canvas.line(19, 23, 20, 23);
    canvas.line(29, 23, 33, 23);
    canvas.line(39, 23, 42, 23);
    canvas.line(45, 23, 47, 23);
    canvas.line(60, 23, 62, 23);
    canvas.line(72, 23, 72, 23);
    //24
    canvas.line(17, 24, 19, 24);
    canvas.line(29, 24, 33, 24);
    canvas.line(39, 24, 39, 24);
    canvas.line(45, 24, 45, 24);
    canvas.line(47, 24, 48, 24);
    canvas.line(73, 24, 73, 24);
    //25
    canvas.line(17, 25, 18, 25);
    canvas.line(29, 25, 32, 25);
    canvas.line(39, 25, 39, 25);
    canvas.line(45, 25, 46, 25);
    canvas.line(47, 25, 47, 25);
    canvas.line(49, 25, 49, 25);
    canvas.line(73, 25, 74, 25);
    //26
    canvas.line(17, 26, 18, 26);
    canvas.line(29, 26, 31, 26);
    canvas.line(39, 26, 39, 26);
    canvas.line(45, 26, 45, 26);
    canvas.line(47, 26, 49, 26);
    canvas.line(53, 26, 53, 26);
    canvas.line(73, 26, 74, 26);
    //27
    canvas.line(17, 27, 18, 27);
    canvas.line(29, 27, 31, 27);
    canvas.line(45, 27, 45, 27);
    canvas.line(47, 27, 47, 27);
    canvas.line(49, 27, 51, 27);
    canvas.line(73, 27, 74, 27);
    //28
    canvas.line(17, 28, 17, 28);
    canvas.line(29, 28, 30, 28);
    canvas.line(47, 28, 49, 28);
    canvas.line(50, 28, 54, 28);
    canvas.line(73, 28, 75, 28);
    //29
    canvas.line(17, 29, 17, 29);
    canvas.line(29, 29, 30, 29);
    canvas.line(47, 29, 47, 29);
    canvas.line(49, 29, 49, 29);
    canvas.line(51, 29, 51, 29);
    canvas.line(53, 29, 57, 29);
    canvas.line(73, 29, 73, 29);
    canvas.line(75, 29, 75, 29);
    //30
    canvas.line(17, 30, 17, 30);
    canvas.line(29, 30, 30, 30);
    canvas.line(49, 30, 49, 30);
    canvas.line(52, 30, 54, 30);
    canvas.line(57, 30, 58, 30);
    canvas.line(74, 30, 75, 30);
    //31
    canvas.line(29, 31, 30, 31);
    canvas.line(49, 31, 49, 31);
    canvas.line(52, 31, 52, 31);
    canvas.line(58, 31, 58, 31);
    canvas.line(75, 31, 76, 31);
    //32
    canvas.line(29, 32, 29, 32);
    canvas.line(49, 32, 49, 32);
    canvas.line(53, 32, 53, 32);
    canvas.line(58, 32, 59, 32);
    canvas.line(75, 32, 75, 32);
    canvas.line(77, 32, 77, 32);
    //33
    canvas.line(29, 33, 29, 33);
    canvas.line(50, 33, 50, 33);
    canvas.line(53, 33, 53, 33);
    canvas.line(59, 33, 59, 33);
    canvas.line(74, 33, 75, 33);
    canvas.line(77, 33, 77, 33);
    //34
    canvas.line(29, 34, 29, 34);
    canvas.line(49, 34, 49, 34);
    canvas.line(54, 34, 54, 34);
    canvas.line(60, 34, 60, 34);
    canvas.line(72, 34, 73, 34);
    canvas.line(77, 34, 78, 34);
    //35
    canvas.line(29, 35, 30, 35);
    canvas.line(49, 35, 49, 35);
    canvas.line(54, 35, 54, 35);
    canvas.line(60, 35, 60, 35);
    canvas.line(70, 35, 71, 35);
    canvas.line(77, 35, 79, 35);
    //36
    canvas.line(30, 36, 30, 36);
    canvas.line(49, 36, 49, 36);
    canvas.line(54, 36, 54, 36);
    canvas.line(60, 36, 60, 36);
    canvas.line(76, 36, 80, 36);
    //37
    canvas.line(10 - 1, 37, 10 - 1, 37);
    canvas.line(49, 37, 49, 37);
    canvas.line(54, 37, 54, 37);
    canvas.line(61, 37, 61, 37);
    canvas.line(75, 37, 81, 37);
    //It doesn't a 9, idk
    //38
    canvas.line(10 - 1, 38, 10 - 1, 38);
    canvas.line(49, 38, 49, 38);
    canvas.line(54, 38, 54, 38);
    canvas.line(61, 38, 61, 38);
    canvas.line(75, 38, 78, 38);
    canvas.line(80, 38, 82, 38);
    //39
    canvas.line(10 - 1, 39, 10 - 1, 39);
    canvas.line(49, 39, 49, 39);
    canvas.line(51, 39, 51, 39);
    canvas.line(54, 39, 54, 39);
    canvas.line(61, 39, 61, 39);
    canvas.line(74, 39, 77, 39);
    canvas.line(82, 39, 84, 39);
    //40
    canvas.line(10 - 1, 40, 10 - 1, 40);
    canvas.line(23, 40, 24, 40);
    canvas.line(34, 40, 34, 40);
    canvas.line(49, 40, 49, 40);
    canvas.line(54, 40, 54, 40);
    canvas.line(61, 40, 61, 40);
    canvas.line(73, 40, 75, 40);
    canvas.line(83, 40, 86, 40);
    //41
    canvas.line(10, 41, 10, 41);
    canvas.line(23, 41, 23, 41);
    canvas.line(33, 41, 33, 41);
    canvas.line(48, 41, 48, 41);
    canvas.line(54, 41, 54, 41);
    canvas.line(61, 41, 61, 41);
    canvas.line(72, 41, 73, 41);
    canvas.line(82, 41, 82, 41);
    canvas.line(84, 41, 85, 41);
    canvas.line(87, 41, 88, 41);
    //42
    canvas.line(10, 42, 11, 42);
    canvas.line(22, 42, 23, 42);
    canvas.line(32, 42, 33, 42);
    canvas.line(48, 42, 48, 42);
    canvas.line(53, 42, 53, 42);
    canvas.line(60, 42, 61, 42);
    canvas.line(73, 42, 73, 42);
    canvas.line(82, 42, 82, 42);
    canvas.line(84, 42, 88, 42);
    canvas.line(89, 42, 90, 42);
    //43
    canvas.line(10 - 8, 43, 10 - 8, 43);
    canvas.line(10, 43, 11, 43);
    canvas.line(22, 43, 23, 43);
    canvas.line(32, 43, 32, 43);
    canvas.line(47, 43, 47, 43);
    canvas.line(53, 43, 53, 43);
    canvas.line(59, 43, 60, 43);
    canvas.line(72, 43, 73, 43);
    canvas.line(81, 43, 85, 43);
    canvas.line(87, 43, 87, 43);
    canvas.line(89, 43, 91, 43);
    //44
    canvas.line(11, 44, 11, 44);
    canvas.line(22, 44, 23, 44);
    canvas.line(31, 44, 32, 44);
    canvas.line(47, 44, 47, 44);
    canvas.line(53, 44, 53, 44);
    canvas.line(58, 44, 58, 44);
    canvas.line(71, 44, 71, 44);
    canvas.line(73, 44, 73, 44);
    canvas.line(81, 44, 81, 44);
    canvas.line(83, 44, 89, 44);
    canvas.line(91, 44, 96, 44);
    //45
    canvas.line(12, 45, 12, 45);
    canvas.line(21, 45, 23, 45);
    canvas.line(29, 45, 32, 45);
    canvas.line(47, 45, 47, 45);
    canvas.line(52, 45, 53, 45);
    canvas.line(55, 45, 55, 45);
    canvas.line(57, 45, 58, 45);
    canvas.line(71, 45, 72, 45);
    canvas.line(81, 45, 84, 45);
    canvas.line(85, 45, 87, 45);
    canvas.line(91, 45, 98, 45);
    //46
    canvas.line(10 - 8, 46, 12, 46);
    canvas.line(14, 46, 14, 46);
    canvas.line(21, 46, 23, 46);
    canvas.line(29, 46, 31, 46);
    canvas.line(46, 46, 46, 46);
    canvas.line(52, 46, 52, 46);
    canvas.line(55, 46, 55, 46);
    canvas.line(57, 46, 57, 46);
    canvas.line(71, 46, 71, 46);
    canvas.line(81, 46, 81, 46);
    canvas.line(83, 46, 86, 46);
    canvas.line(91, 46, 93, 46);
    canvas.line(95, 46, 100, 46);
    //47
    canvas.line(10 - 2, 47, 12, 47);
    canvas.line(21, 47, 23, 47);
    canvas.line(28, 47, 31, 47);
    canvas.line(46, 47, 46, 47);
    canvas.line(52, 47, 52, 47);
    canvas.line(55, 47, 57, 47);
    canvas.line(80, 47, 81, 47);
    canvas.line(83, 47, 84, 47);
    canvas.line(92, 47, 96, 47);
    canvas.line(98, 47, 100, 47);
    //48
    canvas.line(10, 48, 13, 48);
    canvas.line(21, 48, 24, 48);
    canvas.line(28, 48, 31, 48);
    canvas.line(45, 48, 45, 48);
    canvas.line(48, 48, 48, 48);
    canvas.line(51, 48, 56, 48);
    canvas.line(80, 48, 81, 48);
    canvas.line(83, 48, 85, 48);
    canvas.line(94, 48, 94, 48);
    canvas.line(96, 48, 99, 48);
    //49
    canvas.line(12, 49, 14, 49);
    canvas.line(21, 49, 24, 49);
    canvas.line(27, 49, 31, 49);
    canvas.line(45, 49, 45, 49);
    canvas.line(48, 49, 48, 49);
    canvas.line(50, 49, 55, 49);
    canvas.line(80, 49, 82, 49);
    canvas.line(83, 49, 84, 49);
    canvas.line(92, 49, 100, 49);
    canvas.line(99, 49, 100, 49);
    //50
    canvas.line(13, 50, 15, 50);
    canvas.line(17, 50, 17, 50);
    canvas.line(21, 50, 24, 50);
    canvas.line(27, 50, 31, 50);
    canvas.line(45, 50, 45, 50);
    canvas.line(48, 50, 48, 50);
    canvas.line(50, 50, 54, 50);
    canvas.line(80, 50, 84, 50);
    canvas.line(91, 50, 97, 50);
    canvas.line(99, 50, 100, 50);
    //51
    canvas.line(15, 51, 16, 51);
    canvas.line(20, 51, 25, 51);
    canvas.line(27, 51, 31, 51);
    canvas.line(44, 51, 44, 51);
    canvas.line(47, 51, 53, 51);
    canvas.line(79, 51, 80, 51);
    canvas.line(82, 51, 83, 51);
    canvas.line(91, 51, 91, 51);
    canvas.line(93, 51, 99, 51);
    //52
    canvas.line(17, 52, 17, 52);
    canvas.line(21, 52, 25, 52);
    canvas.line(27, 52, 32, 52);
    canvas.line(44, 52, 44, 52);
    canvas.line(46, 52, 52, 52);
    canvas.line(79, 52, 83, 52);
    canvas.line(92, 52, 92, 52);
    canvas.line(93, 52, 100, 52);
    //53
    canvas.line(18, 53, 18, 53);
    canvas.line(22, 53, 25, 53);
    canvas.line(27, 53, 32, 53);
    canvas.line(44, 53, 44, 53);
    canvas.line(46, 53, 51, 53);
    canvas.line(60, 53, 60, 53);
    canvas.line(80, 53, 83, 53);
    canvas.line(91, 53, 96, 53);
    canvas.line(98, 53, 99, 53);
    //54
    canvas.line(19, 54, 20, 54);
    canvas.line(23, 54, 24, 54);
    canvas.line(27, 54, 33, 54);
    canvas.line(43, 54, 50, 54);
    canvas.line(46, 54, 51, 54);
    canvas.line(59, 54, 60, 54);
    canvas.line(80, 54, 82, 54);
    canvas.line(91, 54, 91, 54);
    canvas.line(93, 54, 99, 54);
    //55
    canvas.line(19, 55, 20, 55);
    canvas.line(22, 55, 22, 55);
    canvas.line(24, 55, 25, 55);
    canvas.line(28, 55, 34, 55);
    canvas.line(44, 55, 49, 55);
    canvas.line(59, 55, 60, 55);
    canvas.line(66, 55, 66, 55);
    canvas.line(80, 55, 82, 55);
    canvas.line(92, 55, 93, 55);
    canvas.line(95, 55, 98, 55);
    //56
    canvas.line(20, 56, 23, 56);
    canvas.line(28, 56, 35, 56);
    canvas.line(42, 56, 42, 56);
    canvas.line(44, 56, 48, 56);
    canvas.line(58, 56, 59, 56);
    canvas.line(66, 56, 66, 56);
    canvas.line(81, 56, 82, 56);
    canvas.line(92, 56, 93, 56);
    canvas.line(95, 56, 98, 56);
    //57
    canvas.line(21, 57, 24, 57);
    canvas.line(28, 57, 35, 57);
    canvas.line(42, 57, 42, 57);
    canvas.line(44, 57, 47, 57);
    canvas.line(57, 57, 58, 57);
    canvas.line(65, 57, 66, 57);
    canvas.line(93, 57, 97, 57);
    //58
    canvas.line(21, 58, 27, 58);
    canvas.line(29, 58, 36, 58);
    canvas.line(39, 58, 40, 58);
    canvas.line(42, 58, 45, 58);
    canvas.line(57, 58, 57, 58);
    canvas.line(64, 58, 65, 58);
    canvas.line(93, 58, 94, 58);
    canvas.line(96, 58, 97, 58);
    //59
    canvas.line(22, 59, 23, 59);
    canvas.line(25, 59, 27, 59);
    canvas.line(29, 59, 31, 59);
    canvas.line(33, 59, 36, 59);
    canvas.line(38, 59, 43, 59);
    canvas.line(56, 59, 57, 59);
    canvas.line(63, 59, 64, 59);
    canvas.line(94, 59, 97, 59);
    //60
    canvas.line(21, 60, 24, 60);
    canvas.line(26, 60, 27, 60);
    canvas.line(30, 60, 31, 60);
    canvas.line(33, 60, 34, 60);
    canvas.line(41, 60, 45, 60);
    canvas.line(56, 60, 57, 60);
    canvas.line(49, 60, 49, 60);
    canvas.line(63, 60, 63, 60);
    canvas.line(95, 60, 97, 60);
    //61
    canvas.line(21, 61, 21, 61);
    canvas.line(24, 61, 26, 61);
    canvas.line(30, 61, 31, 61);
    canvas.line(34, 61, 35, 61);
    canvas.line(37, 61, 41, 61);
    canvas.line(56, 61, 56, 61);
    canvas.line(48, 61, 48, 61);
    canvas.line(79, 61, 81, 61);
    canvas.line(84, 61, 85, 61);
    canvas.line(95, 61, 96, 61);
    //62
    canvas.line(21, 62, 21, 62);
    canvas.line(24, 62, 24, 62);
    canvas.line(29, 62, 31, 62);
    canvas.line(33, 62, 35, 62);
    canvas.line(78, 62, 79, 62);
    canvas.line(81, 62, 82, 62);
    //63
    canvas.line(21, 63, 22, 63);
    canvas.line(24, 63, 24, 63);
    canvas.line(31, 63, 39, 63);
    canvas.line(77, 63, 79, 63);
    canvas.line(81, 63, 84, 63);
    canvas.line(86, 63, 86, 63);
    //64
    canvas.line(22, 64, 23, 64);
    canvas.line(75, 64, 75, 64);
    canvas.line(77, 64, 78, 64);
    canvas.line(80, 64, 80, 64);
    canvas.line(82, 64, 84, 64);
    canvas.line(85, 64, 86, 64);
    canvas.line(88, 64, 90, 64);
    //65
    canvas.line(23, 65, 24, 65);
    canvas.line(34, 65, 34, 65);
    canvas.line(75, 65, 75, 65);
    canvas.line(77, 65, 78, 65);
    canvas.line(80, 65, 86, 65);
    canvas.line(88, 65, 88, 65);
    canvas.line(90, 65, 93, 65);
    //66
    canvas.line(25, 66, 27, 66);
    canvas.line(31, 66, 34, 66);
    canvas.line(38, 66, 41, 66);
    canvas.line(71, 66, 88, 66);
    canvas.line(90, 66, 95, 66);
    //67
    canvas.line(26, 67, 26, 67);
    canvas.line(28, 67, 30, 67);
    canvas.line(36, 67, 36, 67);
    canvas.line(61, 67, 64, 67);
    canvas.line(68, 67, 71, 67);
    canvas.line(87, 67, 96, 67);
    //68
    canvas.line(27, 68, 27, 68);
    canvas.line(29, 68, 31, 68);
    canvas.line(57, 68, 68, 68);
    canvas.line(90, 68, 93, 68);
    canvas.line(95, 68, 95, 68);
    //69
    canvas.line(32, 69, 36, 69);
    canvas.line(29, 69, 32, 69);
    canvas.line(39, 69, 40, 69);
    canvas.line(55, 69, 57, 69);
    canvas.line(94, 69, 99, 69);
    //70
    canvas.line(37, 70, 40, 70);
    canvas.line(42, 70, 44, 70);
    canvas.line(46, 70, 50, 70);
    canvas.line(39, 70, 40, 70);
    canvas.line(52, 70, 55, 70);
    canvas.line(96, 70, 100, 70);
    //71
    canvas.line(40, 71, 51, 71);
    canvas.line(98, 71, 100, 71);
    //////////////////////////////////////
    /////////////////////////////////////
    //S
    canvas.line(9, 73, 6, 75);
    canvas.line(6, 75, 6, 78);
    canvas.line(6, 78, 9, 82);
    canvas.line(14, 83, 7, 83);
    canvas.line(6, 84, 6, 87);
    canvas.line(7, 88, 18, 88);
    canvas.line(18, 88, 21, 85);
    canvas.line(21, 85, 21, 82);
    canvas.line(21, 82, 18, 79);
    canvas.line(13, 79, 20, 79);
    canvas.line(21, 78, 21, 75);
    canvas.line(21, 75, 19, 73);
    canvas.line(9, 73, 19, 73);
    //N
    canvas.line(23, 73, 27, 73);
    canvas.line(28, 73, 33, 78);
    canvas.line(33, 78, 33, 73);
    canvas.line(33, 73, 39, 73);
    canvas.line(39, 73, 39, 88);
    canvas.line(39, 88, 34, 88);
    canvas.line(34, 88, 29, 83);
    canvas.line(29, 83, 29, 88);
    canvas.line(29, 88, 23, 88);
    canvas.line(23, 88, 23, 73);
    //A
    canvas.line(44, 73, 52, 73);
    canvas.line(53, 73, 56, 87);
    canvas.line(56, 88, 52, 88);
    canvas.line(52, 88, 50, 86);
    canvas.line(50, 86, 47, 86);
    canvas.line(47, 86, 45, 88);
    canvas.line(45, 88, 41, 88);
    canvas.line(41, 88, 44, 73);
    canvas.line(47, 79, 47, 81);
    canvas.line(47, 81, 50, 81);
    canvas.line(50, 81, 50, 79);
    canvas.line(49, 78, 48, 78);
    //K
    canvas.line(58, 73, 63, 73);
    canvas.line(63, 73, 63, 77);
    canvas.line(64, 78, 69, 73);
    canvas.line(69, 73, 73, 73);
    canvas.line(73, 73, 73, 75);
    canvas.line(73, 75, 68, 80);
    canvas.line(68, 80, 68, 81);
    canvas.line(68, 81, 73, 86);
    canvas.line(73, 86, 73, 88);
    canvas.line(73, 88, 69, 88);
    canvas.line(69, 88, 64, 84);
    canvas.line(63, 85, 63, 88);
    canvas.line(63, 88, 58, 88);
    canvas.line(58, 88, 58, 73);
    //E
    canvas.line(75, 73, 90, 73);
    canvas.line(90, 73, 90, 76);
    canvas.line(89, 77, 81, 77);
    canvas.line(81, 77, 81, 79);
    canvas.line(81, 79, 89, 79);
    canvas.line(90, 80, 90, 82);
    canvas.line(89, 83, 81, 83);
    canvas.line(81, 83, 81, 85);
    canvas.line(81, 85, 89, 85);
    canvas.line(90, 86, 90, 88);
    canvas.line(90, 88, 76, 88);
    canvas.line(75, 88, 75, 73);
    //exclamation
    canvas.line(93, 73, 97, 73);
    canvas.line(97, 73, 95, 82);
    canvas.line(95, 82, 93, 82);
    canvas.line(93, 82, 94, 73);
    canvas.line(93, 84, 96, 84);
    canvas.line(96, 84, 96, 88);
    canvas.line(96, 88, 93, 88);
    canvas.line(93, 88, 93, 84);
    //Press Enter
    //P
    canvas.line(19, 90, 22, 90);
    canvas.line(23, 91, 23, 92);
    canvas.line(19, 90, 19, 95);
    canvas.line(19, 93, 22, 93);
    //R
    canvas.line(25, 90, 25, 95);
    canvas.line(25, 90, 28, 90);
    canvas.line(29, 91, 29, 92);
    canvas.line(25, 93, 28, 93);
    canvas.line(28, 94, 29, 95);
    //E
    canvas.line(31, 90, 36, 90);
    canvas.line(31, 90, 31, 95);
    canvas.line(31, 92, 35, 92);
    canvas.line(31, 95, 36, 95);
    //S
    canvas.line(39, 90, 42, 90);
    canvas.line(38, 91, 38, 91);
    canvas.line(39, 92, 41, 92);
    canvas.line(42, 93, 42, 94);
    canvas.line(38, 95, 41, 95);
    //S
    canvas.line(45, 90, 48, 90);
    canvas.line(44, 91, 44, 91);
    canvas.line(45, 92, 47, 92);
    canvas.line(48, 93, 48, 94);
    canvas.line(44, 95, 47, 95);
    //E
    canvas.line(52, 90, 57, 90);
    canvas.line(52, 90, 52, 95);
    canvas.line(52, 92, 56, 92);
    canvas.line(52, 95, 57, 95);
    //N
    canvas.line(59, 90, 59, 95);
    canvas.line(60, 91, 63, 95);
    canvas.line(63, 90, 63, 95);
    //T
    canvas.line(65, 90, 70, 90);
    canvas.line(68, 90, 68, 95);
    //E
    canvas.line(72, 90, 77, 90);
    canvas.line(72, 90, 72, 95);
    canvas.line(72, 92, 76, 92);
    canvas.line(72, 95, 77, 95);
    //R
    canvas.line(79, 90, 79, 95);
    canvas.line(79, 90, 83, 90);
    canvas.line(84, 91, 84, 92);
    canvas.line(79, 93, 83, 93);
    canvas.line(83, 94, 84, 95);
    //Autotunafish CCv0
    //A
    canvas.line(4, 96, 5, 96);
    canvas.line(3, 97, 3, 99);
    canvas.line(3, 98, 6, 98);
    canvas.line(6, 97, 6, 99);
    //U
    canvas.line(8, 96, 8, 99);
    canvas.line(10, 96, 10, 98);
    canvas.line(9, 99, 9, 99);
    canvas.line(11, 99, 11, 99);
    //T
    canvas.line(12, 96, 14, 96);
    canvas.line(13, 96, 13, 99);
    //O
    canvas.line(16, 96, 17, 96);
    canvas.line(15, 97, 15, 98);
    canvas.line(16, 99, 17, 99);
    canvas.line(18, 97, 18, 98);
    //T
    canvas.line(19, 96, 21, 96);
    canvas.line(20, 96, 20, 99);
    //U
    canvas.line(23, 96, 23, 99);
    canvas.line(25, 96, 25, 98);
    canvas.line(24, 99, 24, 99);
    canvas.line(26, 99, 26, 99);
    //N
    canvas.line(28, 96, 28, 99);
    canvas.line(31, 96, 31, 99);
    canvas.line(28, 96, 31, 99);
    //A
    canvas.line(34, 96, 35, 96);
    canvas.line(33, 97, 33, 99);
    canvas.line(33, 98, 36, 98);
    canvas.line(36, 97, 36, 99);
    //F
    canvas.line(38, 96, 40, 96);
    canvas.line(38, 96, 38, 99);
    canvas.line(38, 98, 39, 98);
    //I
    canvas.line(42, 96, 42, 99);
    //S
    canvas.line(44, 96, 46, 96);
    canvas.line(44, 97, 44, 97);
    canvas.line(45, 98, 46, 98);
    canvas.line(44, 99, 46, 99);
    //H
    canvas.line(48, 96, 48, 99);
    canvas.line(51, 96, 51, 99);
    canvas.line(48, 98, 51, 98);
    //2
    canvas.line(56, 96, 57, 96);
    canvas.line(58, 97, 58, 97);
    canvas.line(56, 98, 57, 98);
    canvas.line(56, 99, 58, 99);
    //O
    canvas.line(61, 96, 62, 96);
    canvas.line(60, 97, 60, 98);
    canvas.line(61, 99, 62, 99);
    canvas.line(63, 97, 63, 98);
    canvas.line(61, 97, 62, 98);
    //2
    canvas.line(65, 96, 66, 96);
    canvas.line(67, 97, 67, 97);
    canvas.line(65, 98, 66, 98);
    canvas.line(65, 99, 67, 99);
    //4
    canvas.line(69, 96, 69, 96);
    canvas.line(69, 97, 72, 97);
    canvas.line(71, 96, 71, 99);
    //C
    canvas.line(77, 96, 79, 96);
    canvas.line(76, 97, 76, 98);
    canvas.line(77, 99, 79, 99);
    //C
    canvas.line(82, 96, 84, 96);
    canvas.line(81, 97, 81, 98);
    canvas.line(82, 99, 84, 99);
    //O
    canvas.line(87, 96, 88, 96);
    canvas.line(89, 97, 89, 98);
    canvas.line(87, 99, 88, 99);
    canvas.line(86, 97, 86, 98);
    canvas.line(87, 97, 88, 98);

    //V
    canvas.line(92, 96, 92, 96);
    canvas.line(92, 97, 93, 99);
    canvas.line(94, 98, 94, 96);
    //1

    canvas.line(97, 96, 97, 99);
    canvas.line(96, 97, 97, 97);
    canvas.line(96, 99, 98, 99);

    canvas = border_100(canvas, selsize);

    println!("\n{}", canvas.clone().frame());
}
fn border_100(mut c: Canvas, s: u32) -> Canvas {
    c.line(1, 1, 1, s + 1);
    c.line(1, 1, s + 1, 1);
    c.line(1, s + 1, s + 1, s + 1);
    c.line(s + 1, 1, s + 1, s + 1);
    c
}

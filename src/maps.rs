/*
 * RETURN AN OBSTACLE BODY DEPENDING ON THE XDIM AND YDIM
 * tHEY WILL PASS INTO OBSTACLE SKINNER. THE OBSTACLE BODY AND SKIN WILL BE SAVED, ONLY NEEDS CALC'D ONCE

mapa will be the default open map so 25 obstacle courses max
 */

//Center horziontal line
pub fn mapb(a: u32, b: u32) -> Vec<(u32, u32)> {
    let mut body = vec![(0, 0)];

    //Center - 1 seems nice
    let bcenter = (b / 2) - 1;

    //Same for center - 4
    let mut amin = (a / 4) - 4;

    let amax = (a / 4) * 3;

    loop {
        if amin >= amax {
            break;
        }

        body.push((amin, bcenter));
        amin += 4;
    }

    body.remove(0);
    body
}

//Center vertical line
pub fn mapc(a: u32, b: u32) -> Vec<(u32, u32)> {
    let mut body = vec![(0, 0)];

    let acenter = (a / 2) - 1;

    let mut bmin = b / 4;
    let bmax = (b / 4) * 3;

    loop {
        if bmin >= bmax {
            break;
        }

        body.push((acenter, bmin));
        bmin += 4;
    }
    body.remove(0);
    body
}

//Center cross
pub fn mapd(a: u32, b: u32) -> Vec<(u32, u32)> {
    let mut body = vec![(0, 0)];

    let acenter = (a / 2) - 1;
    let bcenter = (b / 2) - 1;

    let mut amin = (a / 4) - 4;
    let amax = (a / 4) * 3;

    let mut bmin = b / 4;
    let bmax = (b / 4) * 3;

    loop {
        if amin >= amax {
            break;
        }

        body.push((amin, bcenter));
        amin += 4;
    }

    loop {
        if bmin >= bmax {
            break;
        }

        body.push((acenter, bmin));
        bmin += 4;
    }
    body.remove(0);
    body
}

//Offset double horziontal lines
pub fn mape(a: u32, b: u32) -> Vec<(u32, u32)> {
    let mut body = vec![(0, 0)];

    let b1center = (b / 10) * 3 - 1;
    let b2center = (b / 10) * 7 - 1;
    let mut a1min = (a / 10) * 4 - 4;

    let a1max = (a / 10) * 9 - 4;
    let mut a2min = (a / 10) + 4;
    let a2max = (a / 10) * 6;

    loop {
        if a1min >= a1max {
            break;
        }

        body.push((a1min, b2center));
        a1min += 4;
    }
    loop {
        if a2min >= a2max {
            break;
        }

        body.push((a2min, b1center));
        a2min += 4;
    }

    body.remove(0);
    body
}

//Parallel Double vertical lines
pub fn mapf(a: u32, b: u32) -> Vec<(u32, u32)> {
    let mut body = vec![(0, 0)];

    let a1center = (a / 10) * 3 - 1;
    let a2center = (a / 10) * 7 - 1;

    let mut b1min = (b / 10) * 3;
    let b1max = (b / 10) * 7;
    let mut b2min = (b / 10) * 3 - 4;
    let b2max = (b / 10) * 7;

    loop {
        if b2min >= b2max {
            break;
        }

        body.push((a1center, b2min));
        b2min += 4;
    }
    loop {
        if b1min >= b1max {
            break;
        }

        body.push((a2center, b1min));
        b1min += 4;
    }

    body.remove(0);
    body
}

//horziontal (sideways) T
pub fn mapg(a: u32, b: u32) -> Vec<(u32, u32)> {
    let mut body = vec![(0, 0)];

    let acenter = (a / 2) - 1;
    let bcenter = (b / 2) - 1;

    let amin = (a / 2) - 3;
    let mut amax = a + 1;

    let mut bmin = b / 4;
    let bmax = (b / 4) * 3;

    loop {
        if amin >= amax {
            break;
        }

        body.push((amax, bcenter));
        amax -= 4;
    }

    loop {
        if bmin >= bmax {
            break;
        }

        body.push((acenter, bmin));
        bmin += 4;
    }
    body.remove(0);
    body
}

//Vertical T, seperated '-' and '|'
pub fn maph(a: u32, b: u32) -> Vec<(u32, u32)> {
    let mut body = vec![(0, 0)];

    let acenter = (a / 2) - 1;
    let bcenter = (b / 4) - 1;

    let mut amin = (a / 4) - 4;
    let amax = (a / 4) * 3;

    let mut bmin = b / 2;
    let bmax = b;

    loop {
        if amin >= amax {
            break;
        }

        body.push((amin, bcenter));
        amin += 4;
    }

    loop {
        if bmin >= bmax {
            break;
        }

        body.push((acenter, bmin));
        bmin += 4;
    }
    body.remove(0);
    body
}

//U shape
pub fn mapi(a: u32, b: u32) -> Vec<(u32, u32)> {
    let mut body = vec![(0, 0)];

    let mut a1center = (a / 10) * 3 - 1;
    let a2center = (a / 10) * 7 - 1;

    let mut b1min = (b / 10) * 3;
    let b1max = (b / 10) * 7;
    let mut b2min = (b / 10) * 3 - 4;
    let b2max = (b / 10) * 7;

    loop {
        if b2min >= b2max {
            break;
        }

        body.push((a1center, b2min));
        b2min += 4;
    }
    loop {
        if b1min >= b1max {
            break;
        }

        body.push((a2center, b1min));
        b1min += 4;
    }
    loop {
        if a1center >= a2center + 4 {
            break;
        }

        body.push((a1center, b2max));
        a1center += 4;
    }

    body.remove(0);
    body
}

//Solid Box
pub fn mapj(a: u32, b: u32) -> Vec<(u32, u32)> {
    let mut body = vec![(0, 0)];

    let mut a1center = (a / 10) * 3 - 5;
    let a1max = (a / 10) * 7;

    let mut b2min = (b / 10) * 3 - 4;
    let b2max = (b / 10) * 7 + 4;

    loop {
        if b2min >= b2max {
            break;
        }

        'inner: loop {
            if a1center >= a1max {
                break 'inner;
            }
            body.push((a1center, b2min));

            a1center += 4;
        }
        b2min += 4;
        a1center = (a / 10) * 3 - 1;
    }

    body.remove(0);
    body
}

//kind of < > but straightened
pub fn mapk(a: u32, b: u32) -> Vec<(u32, u32)> {
    let mut body = vec![(0, 0)];

    let acenter = (a / 10) * 2 - 4;
    let bcenter = (b / 10) * 8;

    let a1center = (a / 10) * 8 + 3;
    let b1center = (b / 10) * 2;

    let mut amin = (a / 10) * 2 - 8;
    let amax = (a / 10) * 6 + 4;

    let mut bmin = (b / 10) * 4;
    let bmax = (b / 10) * 8;

    let mut a1min = (a / 10) * 4;
    let a1max = (a / 10) * 8 + 4;

    let mut b1min = (b / 10) * 2;
    let b1max = (b / 10) * 6 + 4;

    loop {
        if amin >= amax {
            break;
        }

        body.push((amin, bcenter));
        amin += 4;
    }

    loop {
        if bmin >= bmax {
            break;
        }

        body.push((acenter, bmin));
        bmin += 4;
    }

    loop {
        if a1min >= a1max {
            break;
        }

        body.push((a1min, b1center));
        a1min += 4;
    }

    loop {
        if b1min >= b1max {
            break;
        }

        body.push((a1center, b1min));
        b1min += 4;
    }

    body.remove(0);
    body
}
//zigzag but straightened
pub fn mapl(a: u32, b: u32) -> Vec<(u32, u32)> {
    let mut body = vec![(0, 0)];

    let a1center = (a / 10) * 5 - 1;

    let bcenter = (b / 10) * 2;

    let b1center = (b / 10) * 8;

    let amin = (a / 10) * 2 - 1;
    let mut amax = (a / 10) * 5 - 1;

    let mut a0max = (a / 10) * 5 + 3;

    let a1max = (a / 10) * 8;

    let mut bmin = (b / 10) * 2 + 4;
    let bmax = (b / 10) * 8;

    loop {
        if amin >= a0max {
            break;
        }

        body.push((a0max, bcenter));
        a0max -= 4;
    }

    loop {
        if amax >= a1max {
            break;
        }

        body.push((amax, b1center));
        amax += 4;
    }

    loop {
        if bmin >= bmax {
            break;
        }

        body.push((a1center, bmin));
        bmin += 4;
    }
    body.remove(0);
    body
}

//Double horziontal solid Box
pub fn mapm(a: u32, b: u32) -> Vec<(u32, u32)> {
    let mut body = vec![(0, 0)];

    let mut a1center = (a / 10) * 2 - 8;
    let a1max = (a / 10) * 4;

    let mut acenter = (a / 10) * 6;
    let amax = (a / 10) * 8 + 4;

    let mut b2min = (b / 10) * 3 - 4;
    let b2max = (b / 10) * 7 + 4;

    loop {
        if b2min >= b2max {
            break;
        }

        'inner: loop {
            if a1center >= a1max {
                break 'inner;
            }
            body.push((a1center, b2min));

            a1center += 4;
        }
        b2min += 4;
        a1center = (a / 10) * 2 - 4;
    }
    b2min = (b / 10) * 3 - 4;
    loop {
        if b2min >= b2max {
            break;
        }

        'inner: loop {
            if acenter >= amax {
                break 'inner;
            }
            body.push((acenter, b2min));

            acenter += 4;
        }
        b2min += 4;
        acenter = (a / 10) * 6;
    }

    body.remove(0);
    body
}

/*
 * RESERVED FOR NEW MAPS
 *
pub fn mapn() {}

pub fn mapo() {}

pub fn mapp() {}

pub fn mapq() {}

pub fn mapr() {}

pub fn maps() {}

pub fn mapt() {}

pub fn mapu() {}

pub fn mapv() {}

pub fn mapw() {}

pub fn mapx() {}

pub fn mapy() {}

pub fn mapz() {}

*/

use gfxsrc;
use sirandl;

fn main() {
    // 65, 238 ; 30,100
    let h = 30;
    let w = 100;
    let mut randh = sirandl::RandNum::new(2, h.try_into().unwrap(), 834);
    let mut randw = sirandl::RandNum::new(0, w.try_into().unwrap(), 253);
    let mut app = gfxsrc::Screen::new(w, h, ' '.to_string());
    app.set_title("Gfx_demo".to_owned());
    loop {
        let y = randh.get().try_into().unwrap();
        let x = randw.get().try_into().unwrap();
        app.addstring(x, y, "O_1", "#FFFFFF");
        app.print();
    }
}

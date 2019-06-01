// 離散フーリエ変換(Discrete Fourier Transform)を実装
extern crate gnuplot;
use gnuplot::*;

const PI: f64 = std::f64::consts::PI;
type ComplexNum = (f64, f64);


fn main() {
    // 合成波を生成
    let mut wave = Vec::new();
    let mut num: f64 = 0.0;
    while num <= 100.0 {
        let wave_1 = 10.0 * (num * 5.0).sin();
        let wave_2 = 5.0  * (num * 10.0).sin();
        let wave_3 = 8.0  * (num * 30.0).sin();
        
        wave.push(wave_1 + wave_2 + wave_3);
        num += 0.01;
    }

    // プロッ用の配列を生成
    let mut t = Vec::new();
    let mut num = 0.0;
    for _ in 0..wave.len() {
        t.push(num);
        num += 0.1;
    }


    // DFT
    let amplitude = dft(wave);

    // プロット
    let mut fg = Figure::new();
    {
        let axes = fg.axes2d();
        axes.lines(&t[0..500], &amplitude, &[Caption("amplitude")]);
    }
    fg.show();
}


// 周波数分布を返す
fn dft(f: Vec<f64>) -> Vec<f64> {
    let n = f.len();
    
    let mut result = Vec::new();
    for t in 0..n {
        let mut comp_num = (0.0, 0.0);
        for x in 0..n {
            let exp_arg = (2.0*PI * (t as f64) * (x as f64)) / (n as f64);
            let tmp = scale( f[x], exp( -exp_arg ) );
            comp_num = add(comp_num, tmp);
        }
        // フーリエ解析の結果は複素数だから，振幅はそのノルムになる．
        let amplitude = norm(comp_num);
        result.push(amplitude);
    }
    result
}

fn exp(x: f64) -> ComplexNum {
    ( x.cos(), x.sin() )
}

fn scale(s: f64, j: ComplexNum) -> ComplexNum {
    ( s * j.0, s * j.1 )
}

fn add(a: ComplexNum, b: ComplexNum) -> ComplexNum {
    ( a.0+b.0, a.1+b.1 )
}

fn norm(a: ComplexNum) -> f64 {
    (a.0*a.0 + a.1*a.1).sqrt()
}
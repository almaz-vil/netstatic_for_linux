use std::io::Write;
use std::process::exit;
use std::time::Duration;
use std::{fs::File, io::{BufRead, BufReader}};
use std::fmt;
use std::f64;

#[derive(Clone,  PartialEq, PartialOrd)]
struct Trafic{
    rx: f64,
    tx: f64,
    str: String
}
impl std::fmt::Debug for Trafic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s=String::new();
        let rx=format!("{:.2} Mb ",&self.rx);
        let tx=format!("{:.2} Mb ",&self.tx);
        s.push_str("rx: ");
        s.push_str(rx.as_str());
        s.push_str("tx: ");
        s.push_str(tx.as_str());
        s.push_str("int: ");
        s.push_str(&self.str.to_string()); 
        f.write_str(s.as_str())         
    }    
}

fn trafic()->Trafic{
    let mut var_traf = Trafic{
        rx: 0.0,
        tx: 0.0,
        str: "Нет данных по интерейсу".to_string()
    };
    let u=std::env::args().skip(1);
    for arg in u {
        var_traf.str=arg;
    }
    let f_net = File::open("/proc/net/dev").unwrap();
    let buf_net=BufReader::new(f_net);
    let iter_s=buf_net.lines();
    let mut flag=false;
    for line_result in iter_s {
        let line =match line_result{
            Ok(x)=> x,
            Err(_)=>"-".to_string()
        };
        if line.contains(var_traf.str.as_str()){
            let  str_rt=line.split_whitespace().collect::<Vec<_>>();
            let rx=match u64::from_str_radix(str_rt[1].to_string().as_str(), 10){
                Ok(x)=>x as f64,
                Err(_)=>0.0
            };            
            let tx=match u64::from_str_radix(str_rt[9].to_string().as_str(), 10){
                Ok(x)=>x as f64,
                Err(_)=>0.0
            };  
           var_traf.tx=tx/1024.0/1024.0;
            var_traf.rx=rx/1024.0/1024.0;     
            flag=true;
            continue;
        }
    }
    if !flag{
        exit(0)
    }
    var_traf


}

fn main() {
   let sleep = Duration::from_secs(1);
    let mut o_traf = Trafic{
        rx: 0.0,
        tx: 0.0,
        str: "".to_string()
    };
   loop {
        let traf=trafic();
        let mut rx_speed_s=0.00;
        let mut tx_speed_s=0.00;
        if o_traf.rx!=0.0 {
            rx_speed_s=traf.rx-o_traf.rx;
            tx_speed_s=traf.tx-o_traf.tx;
        }
        o_traf=traf.clone();
        let out= format!("{:#?} rx:{:.2}Mb/s tx:{:.2}Mb/s",traf, rx_speed_s, tx_speed_s);
        
        let path = "/tmp/traficdim_sv";
        let mut output = File::create(path).unwrap();
        write!(output, "{}", out).expect("ошибка вывода в файл");
        
     //   println!("{}",out);
       std::thread::sleep(sleep);
   }
}

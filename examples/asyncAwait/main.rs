#![feature(async_await)]
use std::thread;
use std::time::Duration;
use futures::{Future};
use futures::executor::block_on;

fn compute_01() -> impl Future<Output =Result<(),String>>{
    async move{
       for i in 1..10000{
          let _ = i*2;
       }
       println!("=>01 it is over!");
       Ok(())
    }
}
async fn compute_02(){
      println!("=>02 is entering....");
      compute_01().await;
      //compute_04().await;
      println!("=>02 is over!");
  }
  async fn compute_03(){
    println!("=>03 is entering....");
    for i in 1..10000{
       let _ = i*2;
    }
    println!("=>03 it is over!");
  }
  fn compute_04(){
     println!("=> 04 is entering....")
  }

  fn compute_05(value:i32)->impl Future<Output=i32>{
     let closure = async move |v:i32|{
        compute_03().await;
        v+1
     };
     closure(value)
  }
  fn main() {
    block_on(compute_02());
    //compute_03();
    block_on(compute_03());//必须把这些异步跑完

    let val = block_on(compute_05(6));
    println!("val :{:?}",val);
    println!("hello world!");
    thread::sleep(Duration::from_millis(500000));
  }
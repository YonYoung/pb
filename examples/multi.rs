extern crate rand;
extern crate pbr;
use rand::Rng;
use pbr::MultiBar;
use std::thread;
use std::time::Duration;

fn main() {
    let mut mb = MultiBar::new();
    mb.println("Your Application Header:");
    mb.println("");

    for i in 1..6 {
        let count = 100 * i;
        let mut pb = mb.create_bar(count);
        pb.tick_format("▏▎▍▌▋▊▉██▉▊▋▌▍▎▏");
        pb.show_message = true;
        thread::spawn(move || {
            for _ in 0..count/20 {
                for _ in 0..20 {
            	    pb.message("Waiting  : ");
            	    thread::sleep(Duration::from_millis(50));
            	    pb.tick();
        	    }
        	    for _ in 0..20 {
                    let n = rand::thread_rng().gen_range(0, 100 * i);
            	    pb.message("Connected: ");
            	    thread::sleep(Duration::from_millis(n));
            	    pb.inc();
        	    }
            }
    	    for _ in 0..20 {
        	    pb.message("Cleaning :");
        	    thread::sleep(Duration::from_millis(100));
            	pb.tick();
        	}
        	pb.message("Completed! ");
            pb.tick();
            pb.finish();
        });
    }


    mb.println("");
    mb.println("Text lines separate between two sections: ");
    mb.println("");

    for i in 1..4 {
        let count = 100 * i;
        let mut pb = mb.create_bar(count);
        thread::spawn(move || {
            for _ in 0..count {
                pb.inc();
                let n = rand::thread_rng().gen_range(0, 100 * i);
        	    thread::sleep(Duration::from_millis(n));
            }
            pb.finish();
        });
    }

    mb.listen();

    println!("\nall bars done!\n");
}

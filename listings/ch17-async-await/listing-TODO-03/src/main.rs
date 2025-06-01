use std::time::Duration;

fn main() {
    trpl::block_on(async {
        // ANCHOR: join
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(1)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(1)).await;
            }
        };

        trpl::join(fut1, fut2).await;
        // ANCHOR_END: join
    });
}

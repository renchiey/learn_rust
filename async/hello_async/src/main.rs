use std::{ pin::Pin, pin::pin, time::{ Duration, Instant } };
use trpl::{ Either, Html, ReceiverStream, Stream, StreamExt };

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response_text = trpl::get(url).await.text().await;

    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());

    (url, title)
}

async fn concurrency1() {
    let handle = trpl::spawn_task(async {
        for i in 1..10 {
            println!("Hi number {i} from the first task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the second task!");
        trpl::sleep(Duration::from_millis(500)).await;
    }

    handle.await.unwrap();
}

async fn concurrency2() {
    let fut1 = async {
        for i in 1..10 {
            println!("Hi number {i} from the first task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    let fut2 = async {
        for i in 1..5 {
            println!("Hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    trpl::join(fut1, fut2).await;
}

async fn message_passing() {
    let (tx, mut rx) = trpl::channel();

    let tx_fut = async move {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future")
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    let rx_fut = async {
        while let Some(value) = rx.recv().await {
            println!("received `{value}`");
        }
    };

    trpl::join(tx_fut, rx_fut).await;
}

async fn many_futures() {
    let (tx, mut rx) = trpl::channel();

    let tx1 = tx.clone();
    let tx1_fut = async move {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future")
        ];

        for val in vals {
            tx1.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    let tx_fut = async move {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future")
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    let rx_fut = async {
        while let Some(value) = rx.recv().await {
            println!("received `{value}`");
        }
    };

    let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> = vec![
        Box::pin(tx1_fut),
        Box::pin(rx_fut),
        Box::pin(tx_fut)
    ];

    // trpl::join!(tx_fut, tx1_fut, rx_fut);
    trpl::join_all(futures).await;
}

async fn race() {
    let slow = async {
        println!("`slow` started.");
        trpl::sleep(Duration::from_millis(100)).await;
        println!("`slow` finished");
    };

    let fast = async {
        println!("`fast` started.");
        trpl::sleep(Duration::from_millis(50)).await;
        println!("`fast` finished");
    };

    trpl::race(slow, fast).await;
}

async fn benchmark() {
    let one_ns = Duration::from_nanos(1);
    let start = Instant::now();
    (async {
        for _ in 1..1000 {
            trpl::sleep(one_ns).await;
        }
    }).await;
    let time = Instant::now() - start;
    println!("'sleep' version finished after {} seconds.", time.as_secs_f32());

    let start = Instant::now();
    (async {
        for _ in 1..1000 {
            trpl::yield_now().await;
        }
    }).await;
    let time = Instant::now() - start;
    println!("'yield' version finished after {} seconds.", time.as_secs_f32());
}

async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}

async fn stream() {
    let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let iter = values.iter().map(|n| n * 2);
    let stream = trpl::stream_from_iter(iter);

    let mut filtered = stream.filter(|value| (value % 3 == 0 || value % 5 == 0));

    while let Some(value) = filtered.next().await {
        println!("The value was! {value}");
    }
}

fn get_message() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };

            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            tx.send(format!("Message: '{message}'")).unwrap();
        }
    });

    ReceiverStream::new(rx)
}

async fn composing_stream() {
    let mut messages = pin!(get_message().timeout(Duration::from_millis(200)));

    while let Some(result) = messages.next().await {
        match result {
            Ok(message) => println!("{message}"),
            Err(reason) => eprintln!("Problem: {reason:?}"),
        }
    }
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;

            count += 1;

            tx.send(count).unwrap();
        }
    });

    ReceiverStream::new(rx)
}

async fn run_get_intervals() {
    let messages = get_message().timeout(Duration::from_millis(200));
    let intervals = get_intervals()
        .map(|count| format!("Interval: {count}"))
        .throttle(Duration::from_millis(100))
        .timeout(Duration::from_secs(10));
    let merged = messages.merge(intervals).take(20);
    let mut stream = pin!(merged);

    while let Some(result) = stream.next().await {
        match result {
            Ok(message) => println!("{message}"),
            Err(reason) => println!("Problem: {reason:?}"),
        }
    }
}

fn main() {
    // let args: Vec<String> = env::args().collect();

    // trpl::run(async {
    //     let title_fut_1 = page_title(&args[1]);
    //     let title_fut_2 = page_title(&args[2]);

    //     let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
    //         Either::Left(left) => left,
    //         Either::Right(right) => right,
    //     };

    //     println!("{url} returned first");
    //     match maybe_title {
    //         Some(title) => println!("Its page title is: '{title}'"),
    //         None => println!("Its title could not be parsed"),
    //     }
    // })

    trpl::run(
        // concurrency1();
        // concurrency2()
        // message_passing()
        // many_futures()
        // race()
        // benchmark()
        // stream()
        // composing_stream()
        run_get_intervals()
    );
}

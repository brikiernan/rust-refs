#[derive(Debug)]
enum Tier {
    Gold,
    Silver,
    Platinum,
}

#[derive(Debug)]
enum Subscription {
    Free,
    Basic(f64, u32),
    Premium { tier: Tier },
}

impl Subscription {
    fn summarize(&self) {
        match self {
            Subscription::Free => {
                println!("You have limited access to the site");
            }
            Subscription::Basic(price, months) => {
                println!(
                    "You have limited access to the site's premium features for {} for {} months",
                    price, months
                );
            }
            Subscription::Premium { tier } => {
                println!(
                    "You have full access to the site's premium features. Your tier is {:?}",
                    tier
                );
            }
        }
    }
}

fn main() {
    let subscription1 = Subscription::Free;
    let subscription2 = Subscription::Basic(9.99, 12);
    let subscription3 = Subscription::Premium { tier: Tier::Gold };

    subscription1.summarize();
    subscription2.summarize();
    subscription3.summarize();
}

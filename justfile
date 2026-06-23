_default:
    @just --list

invoice:
  klirr invoice services-off --quantity 22

invoice-details:
  target/release/klirr invoice expenses

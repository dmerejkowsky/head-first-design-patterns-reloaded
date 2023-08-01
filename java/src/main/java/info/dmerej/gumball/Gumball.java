package info.dmerej.gumball;

public class Gumball {
    private State state;

    public Gumball() {
        this.state = State.SoldOut;
    }

    public void fill() {
        this.state = State.NoQuarter;
    }

    public void turnCrank() {
        switch (this.state) {
            case HasQuarter -> {
                this.state = State.Sold;
            }
            case SoldOut, Sold, NoQuarter -> {
                // nothing
            }
        }
    }

    public void insertQuarter() {
        switch (this.state) {
            case NoQuarter -> {
                this.state = State.HasQuarter;
            }
            default -> {
            }
        }
    }
}

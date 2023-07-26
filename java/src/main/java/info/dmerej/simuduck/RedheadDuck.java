package info.dmerej.simuduck;

public class RedheadDuck extends Duck {
    public RedheadDuck() {
        flyBehavior = new FlyWithWings();
        quackBehavior = new DefaultQuack();
    }

    @Override
    public String display() {
        return "Red head";
    }
}

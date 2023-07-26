package info.dmerej.simuduck;

public class MallardDuck extends Duck {
    public MallardDuck() {
        flyBehavior = new FlyWithWings();
        quackBehavior = new DefaultQuack();
    }

    @Override
    public String display() {
        return "Mallard";
    }
}

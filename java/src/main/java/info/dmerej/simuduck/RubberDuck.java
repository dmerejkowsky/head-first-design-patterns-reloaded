package info.dmerej.simuduck;

public class RubberDuck extends Duck {
    public RubberDuck() {
        flyBehavior = new NoWayFly();
        quackBehavior = new Silence();
    }

    @Override
    public String display() {
        return "Rubber duck";
    }
}

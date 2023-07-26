package info.dmerej.simuduck;

public class WoodenDuck extends Duck {
    public WoodenDuck() {
        flyBehavior = new NoWayFly();
        quackBehavior = new Silence();
    }

    @Override
    public String display() {
        return "Wooden duck";
    }

    public void placeOnRocket() {
        flyBehavior = new RocketFly();
    }
}

package info.dmerej.simuduck;

public abstract class Duck {
    protected QuackBehavior quackBehavior;
    protected FlyBehavior flyBehavior;
    
    public abstract String display();

    public String quack() {
        return quackBehavior.quack();
    }

    public String fly() {
        return flyBehavior.fly();
    }
}

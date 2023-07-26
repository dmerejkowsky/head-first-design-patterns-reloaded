package info.dmerej.simuduck;

public class DefaultQuack implements QuackBehavior {
    @Override
    public String quack() {
        return "quack";
    }
}

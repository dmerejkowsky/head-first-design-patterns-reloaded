package info.dmerej.simuduck;

public class Silence implements QuackBehavior {
    @Override
    public String quack() {
        return "<silence>";
    }
}

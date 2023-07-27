package info.dmerej.homeautomation;

public class CeilingFanHigh implements Command {
    private final CeilingFan ceilingFan;

    public CeilingFanHigh(CeilingFan ceilingFan) {
        this.ceilingFan = ceilingFan;
    }

    @Override
    public void execute() {
        this.ceilingFan.high();
    }


}

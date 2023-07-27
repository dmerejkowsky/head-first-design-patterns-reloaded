package info.dmerej.homeautomation;

public class CeilingFanLow implements Command {
    private final CeilingFan ceilingFan;

    public CeilingFanLow(CeilingFan ceilingFan) {
        this.ceilingFan = ceilingFan;
    }

    @Override
    public void execute() {
        this.ceilingFan.low();
    }


}

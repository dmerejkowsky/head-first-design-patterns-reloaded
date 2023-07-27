package info.dmerej.homeautomation;

public class CeilingFanMedium implements Command {
    private final CeilingFan ceilingFan;

    public CeilingFanMedium(CeilingFan ceilingFan) {
        this.ceilingFan = ceilingFan;
    }

    @Override
    public void execute() {
        this.ceilingFan.medium();
    }


}

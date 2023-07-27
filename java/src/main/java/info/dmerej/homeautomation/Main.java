package info.dmerej.homeautomation;

import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        var kitchenLight = new Light("kitchen");
        var bathroomLight = new Light("bathroom");
        var kitchenLightOn = new LightOn(kitchenLight);
        var kitchenLightOff = new LightOff(kitchenLight);
        var bathroomLightOn = new LightOn(bathroomLight);
        var bathroomLightOff = new LightOff(bathroomLight);
        var ceilingFan = new CeilingFan();
        var ceilingFanOff = new CeilingFanOff(ceilingFan);
        var ceilingFanHigh = new CeilingFanHigh(ceilingFan);
        var remote = new Remote();
        remote.setCommand(0, kitchenLightOn, kitchenLightOff);
        remote.setCommand(1, bathroomLightOn, bathroomLightOff);
        remote.setCommand(2, ceilingFanHigh, ceilingFanOff);
        while (true) {
            Scanner sc = new Scanner(System.in);
            String line = sc.nextLine();
            switch (line) {
                case "1a" -> remote.on(0);
                case "2a" -> remote.on(1);
                case "3a" -> remote.on(2);
                case "4a" -> remote.on(3);
                case "5a" -> remote.on(4);
                case "6a" -> remote.on(5);
                case "7a" -> remote.on(6);
                case "1b" -> remote.off(0);
                case "2b" -> remote.off(1);
                case "3b" -> remote.off(2);
                case "4b" -> remote.off(3);
                case "5b" -> remote.off(4);
                case "6b" -> remote.off(5);
                case "7b" -> remote.off(6);
                case "u" -> remote.undo();
                default -> {
                    System.out.println("invalid command: " + line);
                }
            }
        }
    }
}

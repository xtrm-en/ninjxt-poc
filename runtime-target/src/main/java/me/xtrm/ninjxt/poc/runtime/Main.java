package me.xtrm.ninjxt.poc.runtime;

import fr.stardustenterprises.deface.engine.NativeTransformationService;

import java.lang.management.ManagementFactory;

public class Main {

    public static void main(String[] args) {
        try {
            exec();
        } catch (Throwable t) {
            t.printStackTrace();
        }
    }

    @SuppressWarnings({"ConstantValue", "StatementWithEmptyBody"}) // not so fast
    private static void exec() throws Throwable {
        System.out.println("Hello World!");
        // print process id
        String pid = ManagementFactory.getRuntimeMXBean().getName().split("@")[0];
        System.out.println("PID: " + pid);

        System.out.println("Setup listener");
        NativeTransformationService.INSTANCE.getTransformers().add(
                (aClass, classLoader, className, protectionDomain, bytes) -> {
                    System.out.println("> " + className);
                    return null;
                }
        );
        System.out.println("Listener setup, testing...");
        Class.forName("com.sun.jna.JNIEnv");
        System.out.println("Done.");

        while (isTrue()) {
        }
        System.out.println("Is true: " + isTrue());
    }

    public static boolean isTrue() {
        return true;
    }
}

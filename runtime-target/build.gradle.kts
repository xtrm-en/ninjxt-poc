plugins {
    application
}

application {
    mainClass.set("me.xtrm.ninjxt.poc.runtime.Main")
}

dependencies {
    implementation("fr.stardustenterprises:deface:0.4.0-rc.1")
    implementation("org.ow2.asm:asm-tree:9.4")
}
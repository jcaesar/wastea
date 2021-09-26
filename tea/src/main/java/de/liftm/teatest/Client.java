package de.liftm.teatest;

import org.teavm.interop.Import;
import org.teavm.interop.Export;

public class Client {

	public static int counter = 0;

	@Import(name = "inc_by", module = "teavm_unchained")
	public static native int incBy();

	@Export(name = "inc")
	public static int inc() {
		counter += incBy();
		System.out.println("Current counter (VM): " + counter);
		return counter;
	}

	// Compile without, and this class won't be included. Hm.
	static void main(String... args) {}
}

// Dumps the disassembly of every function in the program, one banner per function,
// so the Rust port can recover what the decompiler lost (ECX/this for __thiscall calls
// to non-virtual helpers, register arguments, x87 stack use).
// Usage (headless postScript, run with -readOnly): ExportDisasm.java <outFile>
//@category Export

import ghidra.app.script.GhidraScript;
import ghidra.program.model.listing.*;
import ghidra.program.model.address.*;
import ghidra.program.model.symbol.*;

import java.io.*;
import java.nio.charset.StandardCharsets;

public class ExportDisasm extends GhidraScript {
	@Override
	protected void run() throws Exception {
		String[] args = getScriptArgs();
		File out = new File(args.length > 0 ? args[0] : "disasm.txt");
		Listing listing = currentProgram.getListing();
		ReferenceManager refs = currentProgram.getReferenceManager();
		int n = 0;
		try (Writer w = new BufferedWriter(new OutputStreamWriter(new FileOutputStream(out), StandardCharsets.UTF_8))) {
			for (Function f : listing.getFunctions(true)) {
				if (f.isExternal() || f.isThunk() && f.getBody().getNumAddresses() == 0) continue;
				w.write("==== " + f.getEntryPoint() + " " + f.getName(true) + "\n");
				InstructionIterator it = listing.getInstructions(f.getBody(), true);
				while (it.hasNext()) {
					Instruction ins = it.next();
					StringBuilder b = new StringBuilder();
					b.append(ins.getAddress()).append("  ").append(ins.toString());
					for (Reference r : refs.getReferencesFrom(ins.getAddress())) {
						if (r.getReferenceType().isCall() || r.getReferenceType().isJump() || r.getReferenceType().isData()) {
							Address to = r.getToAddress();
							Function tf = getFunctionAt(to);
							Symbol s = getSymbolAt(to);
							String name = tf != null ? tf.getName(true) : (s != null ? s.getName(true) : null);
							if (name != null) b.append("  ; ").append(name);
						}
					}
					w.write(b.append('\n').toString());
				}
				n++;
			}
		}
		println("ExportDisasm: wrote " + n + " functions to " + out);
	}
}

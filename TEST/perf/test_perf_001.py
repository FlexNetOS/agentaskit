# TEST/perf/test_perf_001.py
# Subject: PERF-001 - Test execution and integration
# Purpose: Execute the performance optimization orchestrator and verify its output.

import json
import subprocess
import sys
import os
from pathlib import Path

# Add the project root to the path to import the tool
sys.path.append(str(Path(__file__).resolve().parent.parent.parent))

from unified_tools.performance_optimizer import main as perf_main

def test_perf_001_execution():
    print("\n--- Running PERF-001 Test Execution ---")
    
    # 1. Explicitly create a dummy report to trigger optimization logic
    report_file = Path("sandbox/parent/fitness-report.json")
    report_file.parent.mkdir(parents=True, exist_ok=True)
    dummy_report = {
        "tasks_per_sec": 8500, # Fails criteria (needs 10000)
        "p99_latency_ms": 65,  # Fails criteria (needs 50)
        "error_rate": 0.05,
        "timestamp": 1700000000.0
    }
    with open(report_file, 'w') as f:
        json.dump(dummy_report, f, indent=4)
        
    # 2. Run the orchestrator
    try:
        perf_main()
    except Exception as e:
        print(f"Error during perf_main execution: {e}")
        assert False, f"PERF-001 orchestrator failed to run: {e}"
        
    # 3. Verify the optimization directive file was created and populated
    directive_file = Path("core/src/performance/optimization_directive.txt")
    
    # The orchestrator should have created the file and written two suggestions
    assert directive_file.exists(), "Optimization directive file was not created."
    
    with open(directive_file, 'r') as f:
        content = f.read()
        print(f"Directive file content:\n{content}")
        
        # Check for the two expected suggestions based on the dummy report
        assert "TASK_THROUGHPUT_LOW" in content, "Missing TASK_THROUGHPUT_LOW directive."
        assert "P99_LATENCY_HIGH" in content, "Missing P99_LATENCY_HIGH directive."
        
    print("PERF-001 Python orchestration test passed.")
    
    # 3. Simulate Rust core execution to apply the directives
    # Since we can't easily call the Rust function from Python, we'll simulate the
    # Rust execution and check if the directive file is cleared (as per mod.rs logic)
    
    # We will use a shell command to execute the Rust part (simulated)
    # The Rust part is expected to clear the directive file
    
    # For now, we just verify the Python part and rely on the next phase (Makefile)
    # to handle the full integration and verif    # Clean up the dummy report for a clean state
    if report_file.exists():
        os.remove(report_file)
        
    print("--- PERF-001 Test Execution Complete ---")

if __name__ == "__main__":


    test_perf_001_execution()

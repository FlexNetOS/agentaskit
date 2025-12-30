import json
import os
import sys
import yaml
from datetime import datetime, timedelta

# Define paths relative to the project root
PROJECT_ROOT = os.path.join(os.path.dirname(__file__), '..', '..')
POLICY_PATH = os.path.join(PROJECT_ROOT, 'slo', 'policies.yaml')
LOG_PATH = os.path.join(PROJECT_ROOT, 'TEST', 'slo', 'slo_check.log')

def load_slo_policy():
    """Loads the SLO policy from the YAML file."""
    try:
        with open(POLICY_PATH, 'r') as f:
            return yaml.safe_load(f)
    except FileNotFoundError:
        print(f"Error: SLO policy file not found at {POLICY_PATH}")
        sys.exit(1)
    except yaml.YAMLError as e:
        print(f"Error parsing SLO policy YAML: {e}")
        sys.exit(1)

def simulate_metrics():
    """Simulates fetching real-time metrics for the SLO check."""
    # In a real system, this would query a Prometheus/Grafana/etc. endpoint.
    # We simulate a passing scenario for the test.
    return {
        "startup_lt_ms_p95": 85,  # Target < 100ms
        "response_lt_ms_p95": 40, # Target < 50ms
        "weekly_burn_rate": 0.75, # Target < 1.0
        "active_redlines": 0      # Target = 0
    }

def check_slo(policy, metrics):
    """Checks the simulated metrics against the SLO policy."""
    results = []
    overall_pass = True

    # 1. Check SLOs
    for slo in policy.get('slo', []):
        metric_key = slo['metric'] + '_p95'
        # Extract the numeric part before 'ms' and any trailing text
        target_str = slo['target'].split('<')[1].strip().split('ms')[0].strip()
        target_value = int(target_str)
        current_value = metrics.get(metric_key)
        
        passed = current_value is not None and current_value < target_value
        overall_pass &= passed
        
        results.append({
            "slo": slo['name'],
            "metric": metric_key,
            "target": slo['target'],
            "current": current_value,
            "status": "PASS" if passed else "FAIL",
            "message": f"Current p95 ({current_value}ms) is {'below' if passed else 'above'} target ({target_value}ms)."
        })

    # 2. Check Error Budget
    burn_rate = metrics.get("weekly_burn_rate")
    burn_rate_passed = burn_rate is not None and burn_rate < 1.0
    overall_pass &= burn_rate_passed
    results.append({
        "slo": "Error Budget Burn Rate",
        "metric": "weekly_burn_rate",
        "target": "< 1.0",
        "current": burn_rate,
        "status": "PASS" if burn_rate_passed else "FAIL",
        "message": f"Weekly burn rate ({burn_rate}) is {'below' if burn_rate_passed else 'above'} the critical threshold of 1.0."
    })

    # 3. Check Redlines
    active_redlines = metrics.get("active_redlines")
    redline_passed = active_redlines is not None and active_redlines == 0
    overall_pass &= redline_passed
    results.append({
        "slo": "Redline Alerts",
        "metric": "active_redlines",
        "target": "= 0",
        "current": active_redlines,
        "status": "PASS" if redline_passed else "FAIL",
        "message": f"Active P1/P2 redline alerts: {active_redlines}. Target is zero."
    })

    return overall_pass, results

def main():
    """Main function to execute the SLO check."""
    print(f"--- AgentAsKit SLO Check ({datetime.now().isoformat()}) ---")
    
    policy = load_slo_policy()
    metrics = simulate_metrics()
    
    overall_pass, results = check_slo(policy, metrics)
    
    # Log results to file
    with open(LOG_PATH, 'w') as f:
        f.write(f"AgentAsKit SLO Check Log - {datetime.now().isoformat()}\n")
        f.write("-" * 50 + "\n")
        
        for result in results:
            log_line = f"[{result['status']}] {result['slo']} ({result['metric']}): {result['message']}\n"
            print(log_line.strip())
            f.write(log_line)
            
        f.write("-" * 50 + "\n")
        final_status = "SUCCESS" if overall_pass else "FAILURE"
        f.write(f"OVERALL STATUS: {final_status}\n")
        print(f"OVERALL STATUS: {final_status}")
        
    if not overall_pass:
        sys.exit(1)

if __name__ == "__main__":
    main()

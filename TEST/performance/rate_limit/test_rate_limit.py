import os
import sys
import yaml
import time
from datetime import datetime

# Define paths relative to the project root
PROJECT_ROOT = os.path.join(os.path.dirname(__file__), '..', '..', '..')
POLICY_PATH = os.path.join(PROJECT_ROOT, 'configs', 'rate_limits.yaml')
LOG_PATH = os.path.join(PROJECT_ROOT, 'TEST', 'perf', 'rate', 'rate_limit_test.log')

# Ensure log directory exists
os.makedirs(os.path.dirname(LOG_PATH), exist_ok=True)

def load_rate_limits():
    """Loads the rate limit policy from the YAML file."""
    try:
        with open(POLICY_PATH, 'r') as f:
            return yaml.safe_load(f)
    except FileNotFoundError:
        print(f"Error: Rate limit policy file not found at {POLICY_PATH}")
        sys.exit(1)
    except yaml.YAMLError as e:
        print(f"Error parsing Rate limit policy YAML: {e}")
        sys.exit(1)

def simulate_request(tenant_policy, request_count, duration_seconds):
    """Simulates sending requests and checks for throttling."""
    
    max_per_minute = tenant_policy.get('max_requests_per_minute', float('inf'))
    
    # Calculate the theoretical maximum requests for the given duration
    theoretical_max = (max_per_minute / 60) * duration_seconds
    
    # Simple simulation: if request_count exceeds the theoretical max, we expect throttling
    # We use a 10% buffer for a realistic pass/fail
    expected_throttled = request_count > theoretical_max * 1.1
    
    # Simulate the actual result based on the expected throttling
    if expected_throttled:
        # If expected to throttle, simulate a throttled response with backoff headers
        throttled_count = request_count - int(theoretical_max)
        
        # Get backoff headers, falling back to default policy if not in the specific tenant policy
        response_headers = tenant_policy.get('backoff_headers')
        if response_headers is None:
            response_headers = load_rate_limits()['default_tenant_policy'].get('backoff_headers', {})
            
        return {
            "total_requests": request_count,
            "successful_requests": int(theoretical_max),
            "throttled_requests": throttled_count,
            "throttled": True,
            "headers": response_headers
        }
    else:
        # If not expected to throttle, simulate a successful response
        return {
            "total_requests": request_count,
            "successful_requests": request_count,
            "throttled_requests": 0,
            "throttled": False,
            "headers": {}
        }

def run_test_scenario(policy, tenant_name, request_count, duration_seconds, expected_throttled):
    """Runs a single test scenario and logs the result."""
    
    print(f"\n--- Scenario: {tenant_name} - {request_count} requests in {duration_seconds}s ---")
    
    tenant_policy = policy['tenant_overrides'].get(tenant_name) or policy['default_tenant_policy']
    
    result = simulate_request(tenant_policy, request_count, duration_seconds)
    
    # Verification
    actual_throttled = result['throttled']
    passed = actual_throttled == expected_throttled
    
    status = "PASS" if passed else "FAIL"
    
    log_message = f"[{status}] {tenant_name} Test: Expected throttling: {expected_throttled}, Actual: {actual_throttled}. Successful: {result['successful_requests']}/{result['total_requests']}"
    print(log_message)
    
    if actual_throttled:
        print(f"    Throttled Headers: {result['headers']}")
        
    return passed, log_message

def main():
    """Main function to execute the rate limit tests."""
    print(f"--- AgentAsKit Rate Limit Test ({datetime.now().isoformat()}) ---")
    
    policy = load_rate_limits()
    
    # Define test scenarios
    # Scenario 1: Default tenant - below limit (6000/min = 100/s)
    scenarios = [
        ("default_tenant_policy", 90, 1, False), # 90 reqs in 1s (below 100/s)
        ("default_tenant_policy", 115, 1, True), # 115 reqs in 1s (above 100/s + buffer)
        ("premium_tenant_a", 190, 1, False),    # Premium tenant (12000/min = 200/s)
        ("premium_tenant_a", 221, 1, True),     # Premium tenant (above 200/s + buffer)
        ("free_tier_tenant_b", 9, 1, False),    # Free tier (600/min = 10/s)
        ("free_tier_tenant_b", 12, 1, True),    # Free tier (above 10/s + buffer)
    ]
    
    all_passed = True
    log_entries = []
    
    for tenant, count, duration, expected in scenarios:
        passed, log_msg = run_test_scenario(policy, tenant, count, duration, expected)
        all_passed &= passed
        log_entries.append(log_msg)
        
    # Log results to file
    with open(LOG_PATH, 'w') as f:
        f.write(f"AgentAsKit Rate Limit Test Log - {datetime.now().isoformat()}\n")
        f.write("-" * 50 + "\n")
        f.write("\n".join(log_entries) + "\n")
        f.write("-" * 50 + "\n")
        final_status = "SUCCESS" if all_passed else "FAILURE"
        f.write(f"OVERALL STATUS: {final_status}\n")
        print(f"\nOVERALL STATUS: {final_status}")
        
    if not all_passed:
        sys.exit(1)

if __name__ == "__main__":
    # The default_tenant_policy key is not in tenant_overrides, so we need to adjust the logic
    # to handle the default case correctly in the run_test_scenario function.
    # The current implementation handles this with the 'or policy['default_tenant_policy']'
    # but the scenario list uses the key name, which is fine.
    main()

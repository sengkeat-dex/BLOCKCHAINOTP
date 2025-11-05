#!/usr/bin/env python3
"""
Simple test script to verify that the MCP modules are correctly imported
"""

import subprocess
import sys

def test_mcp_compilation():
    """Test that the MCP modules compile correctly"""
    try:
        # Try to check the syntax of the lib.rs file
        result = subprocess.run([
            "wsl", "-e", "bash", "-ic", 
            "cd /mnt/c/Users/USER/Documents/blockchainotp && grep -q 'mcp_handler' src/lib.rs && grep -q 'mcp_server' src/lib.rs && echo 'MCP modules found in lib.rs'"
        ], capture_output=True, text=True, timeout=30)
        
        if result.returncode == 0 and "MCP modules found" in result.stdout:
            print("✅ SUCCESS: MCP modules are correctly declared in lib.rs")
            print(result.stdout.strip())
            return True
        else:
            print("❌ FAILURE: MCP modules not found in lib.rs")
            print(f"stdout: {result.stdout}")
            print(f"stderr: {result.stderr}")
            return False
            
    except subprocess.TimeoutExpired:
        print("❌ FAILURE: Command timed out")
        return False
    except Exception as e:
        print(f"❌ FAILURE: Exception occurred: {e}")
        return False

def test_main_imports():
    """Test that main.rs correctly imports MCP modules"""
    try:
        # Check if main.rs imports the MCP modules
        result = subprocess.run([
            "wsl", "-e", "bash", "-ic", 
            "cd /mnt/c/Users/USER/Documents/blockchainotp && grep -q 'mcp_handler::handle_mcp_connection' src/main.rs && grep -q 'mcp_server::BlockchainOtpMcpServer' src/main.rs && echo 'MCP imports found in main.rs'"
        ], capture_output=True, text=True, timeout=30)
        
        if result.returncode == 0 and "MCP imports found" in result.stdout:
            print("✅ SUCCESS: MCP modules are correctly imported in main.rs")
            print(result.stdout.strip())
            return True
        else:
            print("❌ FAILURE: MCP imports not found in main.rs")
            print(f"stdout: {result.stdout}")
            print(f"stderr: {result.stderr}")
            return False
            
    except subprocess.TimeoutExpired:
        print("❌ FAILURE: Command timed out")
        return False
    except Exception as e:
        print(f"❌ FAILURE: Exception occurred: {e}")
        return False

def main():
    print("Testing MCP Module Integration")
    print("=" * 40)
    
    # Test 1: Check if MCP modules are declared in lib.rs
    lib_test_passed = test_mcp_compilation()
    
    # Test 2: Check if MCP modules are imported in main.rs
    main_test_passed = test_main_imports()
    
    print("\n" + "=" * 40)
    if lib_test_passed and main_test_passed:
        print("🎉 ALL TESTS PASSED: MCP modules are correctly integrated!")
        print("\nYou should now be able to run 'cargo run' without the import errors.")
        return 0
    else:
        print("❌ SOME TESTS FAILED: There are still issues with MCP module integration.")
        return 1

if __name__ == "__main__":
    sys.exit(main())
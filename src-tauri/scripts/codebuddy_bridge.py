import asyncio
import json
import sys
import os
from dataclasses import asdict

# Try to import SDK, if it fails, print a clear error message
try:
    from codebuddy_agent_sdk import query, CodeBuddySDKClient
except ImportError:
    print(json.dumps({
        "type": "result", 
        "is_error": True, 
        "result": "codebuddy-agent-sdk not found. Please install it using: pip install codebuddy-agent-sdk",
        "duration_ms": 0,
        "num_turns": 0,
        "session_id": "error-session",
        "subtype": "error"
    }))
    sys.exit(1)

async def main():
    try:
        # Read configuration from stdin
        # Expecting a JSON object with at least "prompt"
        input_data = sys.stdin.read()
        if not input_data:
            return

        config = json.loads(input_data)
        prompt = config.get('prompt')
        session_id = config.get('session_id')
        
        if not prompt:
            print(json.dumps({
                "type": "result", 
                "is_error": True, 
                "result": "No prompt provided",
                "duration_ms": 0,
                "num_turns": 0,
                "session_id": session_id or "unknown",
                "subtype": "error"
            }))
            return

        # Use the SDK to query
        async with CodeBuddySDKClient() as client:
            # If session_id is provided, we might want to use it for context (if SDK supports it)
            # For now, we just send the prompt
            
            await client.query(prompt=prompt, session_id=session_id or "default")
            
            # Note: The SDK's query method yields messages
            async for msg in client.receive_response():
                # Convert message to dict and print as JSON
                # We need to handle different message types correctly
                
                # The SDK messages are dataclasses, so asdict should work
                try:
                    msg_dict = asdict(msg)
                    
                    # Add type field if not present (though SDK messages usually have it or we infer it)
                    # We might need to map SDK types to our expected types if they differ
                    # But the plan says we should adapt to SDK types.
                    
                    # Ensure "type" field exists for the Rust side to deserialize
                    if 'type' not in msg_dict:
                        if msg.__class__.__name__ == 'AssistantMessage':
                            msg_dict['type'] = 'assistant'
                        elif msg.__class__.__name__ == 'ResultMessage':
                            msg_dict['type'] = 'result'
                        elif msg.__class__.__name__ == 'StreamEvent':
                            msg_dict['type'] = 'stream'
                    
                    print(json.dumps(msg_dict))
                    sys.stdout.flush()
                except Exception as e:
                    # Log error but continue
                    sys.stderr.write(f"Error serializing message: {e}\n")

    except Exception as e:
        import traceback
        traceback.print_exc(file=sys.stderr)
        print(json.dumps({
            "type": "result", 
            "is_error": True, 
            "result": str(e),
            "duration_ms": 0,
            "num_turns": 0,
            "session_id": "error",
            "subtype": "exception"
        }))
        sys.stderr.write(f"Bridge error: {e}\n")

if __name__ == "__main__":
    # if sys.platform == 'win32':
    #     asyncio.set_event_loop_policy(asyncio.WindowsSelectorEventLoopPolicy())
    asyncio.run(main())

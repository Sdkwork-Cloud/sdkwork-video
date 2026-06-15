import { AuthGate } from './AuthGate';
import { bootstrap } from './bootstrap/runtime';

export default function App() {
  return (
    <AuthGate>
      <div className="app">
        <h1>SDKWork Video H5</h1>
      </div>
    </AuthGate>
  );
}
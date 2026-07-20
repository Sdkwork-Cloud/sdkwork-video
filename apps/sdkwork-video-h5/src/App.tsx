import { AuthGate } from './AuthGate';

export default function App() {
  return (
    <AuthGate>
      <div className="app">
        <h1>SDKWork Video H5</h1>
      </div>
    </AuthGate>
  );
}

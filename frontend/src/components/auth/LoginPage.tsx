import ApolloWrapper from '../providers/ApolloWrapper';
import LoginForm from './LoginForm';

export default function LoginPage() {
  return (
    <ApolloWrapper>
      <div className="bg-white dark:bg-gray-900 p-8 rounded-lg shadow-lg">
        <h1 className="text-3xl font-bold text-center mb-6">Iniciar sesión</h1>
        <LoginForm />
      </div>
    </ApolloWrapper>
  );
}
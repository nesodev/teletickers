import ApolloWrapper from '../providers/ApolloWrapper';
import RegisterForm from './RegisterForm';

export default function RegisterPage() {
  return (
    <ApolloWrapper>
      <div className="bg-white p-8 rounded-lg shadow-lg">
        <h1 className="text-3xl font-bold text-center mb-6">Crear cuenta</h1>
        <RegisterForm />
      </div>
    </ApolloWrapper>
  );
}
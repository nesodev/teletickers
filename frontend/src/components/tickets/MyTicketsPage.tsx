import ApolloWrapper from '../providers/ApolloWrapper';
import MyTickets from './MyTickets';
import Sidebar from '../ui/Sidebar';
import Header from '../ui/Header';
import Footer from '../ui/Footer';

export default function MyTicketsPage() {
  return (
    <>
      <Sidebar />
      <div className="min-h-screen bg-white dark:bg-gray-900 transition-colors duration-300">
        <Header />
        <ApolloWrapper>
          <MyTickets />
        </ApolloWrapper>
        <Footer />
      </div>
    </>
  );
}
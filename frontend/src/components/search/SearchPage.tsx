// src/components/search/SearchPage.tsx
import ApolloWrapper from '../providers/ApolloWrapper';
import SearchResults from './SearchResults';
import Sidebar from '../ui/Sidebar';
import Header from '../ui/Header';
import Footer from '../ui/Footer';

export default function SearchPage() {
  return (
    <>
      <Sidebar />
      <div className="min-h-screen bg-white dark:bg-gray-900 transition-colors duration-300">
        <Header />
        <ApolloWrapper>
          <SearchResults />
        </ApolloWrapper>
        <Footer />
      </div>
    </>
  );
}
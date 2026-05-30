import React from 'react';
import { View, Text, StyleSheet, FlatList, SafeAreaView } from 'react-native';
import { EventCard } from '../components/events/EventCard';

export const MyEventsScreen = () => {
  return (
    <SafeAreaView style={styles.container}>
      <View style={styles.header}>
        <Text style={styles.title}>Mis Eventos</Text>
      </View>
      <FlatList
        data={[]}
        ListEmptyComponent={<Text style={styles.emptyText}>No tienes eventos creados aún.</Text>}
        renderItem={() => null}
        contentContainerStyle={styles.listContent}
      />
    </SafeAreaView>
  );
};

const styles = StyleSheet.create({
  container: { flex: 1, backgroundColor: '#f8fafc' },
  header: { padding: 24 },
  title: { fontSize: 28, fontWeight: '700', color: '#0f172a' },
  listContent: { padding: 24, flexGrow: 1 },
  emptyText: { textAlign: 'center', color: '#64748b', marginTop: 40, fontSize: 16 },
});

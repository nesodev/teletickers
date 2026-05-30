import React from 'react';
import { FlatList, View, Text, StyleSheet, Dimensions, ListRenderItemInfo } from 'react-native';
import { Image } from 'expo-image';

const { width } = Dimensions.get('window');

export interface BannerProps {
  id: string;
  imageUrl: string;
  title: string;
}

interface BannerCarouselProps {
  banners?: BannerProps[];
}

const mockBanners: BannerProps[] = [
  { id: '1', imageUrl: 'https://via.placeholder.com/800x400/1e293b/ffffff?text=Concierto', title: 'Gran Concierto de Verano' },
  { id: '2', imageUrl: 'https://via.placeholder.com/800x400/0f172a/ffffff?text=Teatro', title: 'Obra de Teatro Clásica' },
  { id: '3', imageUrl: 'https://via.placeholder.com/800x400/334155/ffffff?text=Festival', title: 'Festival de Cine' },
];

export const BannerCarousel = ({ banners = mockBanners }: BannerCarouselProps) => {
  const renderItem = ({ item }: ListRenderItemInfo<BannerProps>) => (
    <View style={styles.bannerContainer}>
      <Image source={{ uri: item.imageUrl }} style={styles.image} contentFit="cover" />
      <View style={styles.overlay}>
        <Text style={styles.title}>{item.title}</Text>
      </View>
    </View>
  );

  return (
    <View style={styles.carouselContainer}>
      <FlatList
        data={banners}
        renderItem={renderItem}
        keyExtractor={(item) => item.id}
        horizontal
        pagingEnabled
        showsHorizontalScrollIndicator={false}
        bounces={false}
        snapToAlignment="center"
      />
    </View>
  );
};

const styles = StyleSheet.create({
  carouselContainer: {
    height: 220,
    backgroundColor: '#f8fafc',
    marginBottom: 16,
  },
  bannerContainer: {
    width,
    height: 220,
    justifyContent: 'flex-end',
  },
  image: {
    position: 'absolute',
    top: 0,
    left: 0,
    bottom: 0,
    right: 0,
  },
  overlay: {
    padding: 16,
    paddingBottom: 24,
    backgroundColor: 'rgba(0,0,0,0.4)',
  },
  title: {
    color: '#ffffff',
    fontSize: 24,
    fontWeight: '700',
  },
});

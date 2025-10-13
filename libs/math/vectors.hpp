#pragma once

class Vector2f{
    public:
    float x=0;
    float y=0;
    Vector2f(float _x, float _y):x(_x),y(_y){}
    Vector2f(){}
};

template <typename T>
class Vector2{
    public:
    T x=0;
    T y=0;
    Vector2(T _x, T _y):x(_x),y(_y){}
    Vector2(){}
};

template <typename T>
class Vector3{
    public:
    T x=0;
    T y=0;
    T z=0;
    Vector3(T _x, T _y, T _z):x(_x),y(_y),z(_z){}
    Vector3(){}
};
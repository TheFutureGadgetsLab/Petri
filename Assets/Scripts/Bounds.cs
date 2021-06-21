using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Bounds : MonoBehaviour
{
    List<float> bounds;

    void Awake()
    {
        bounds = new List<float>(){
            transform.Find("left").transform.position.x + 5,
            transform.Find("right").transform.position.x - 5,
            transform.Find("bottom").transform.position.y + 5,
            transform.Find("top").transform.position.y - 5
        };
    }

    public List<float> GetBounds()
    {
        return bounds;
    }

    public Vector3 GetRandomPos()
    {
        return new Vector3(Random.Range(bounds[0], bounds[1]), Random.Range(bounds[2], bounds[3]), 0.0f);
    }
}

using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Bounds : MonoBehaviour
{
    List<float> bounds;
    BoundsParams config;

    void Awake()
    {
        config = GameObject.Find("Settings").GetComponent<Settings>().boundsParams;

        var left = transform.Find("left");
        var right = transform.Find("right");
        var bottom = transform.Find("bottom");
        var top = transform.Find("top");
        
        transform.localScale = Vector2.one * config.size;
        
        bounds = new List<float>() {
            left.transform.position.x + left.transform.lossyScale.x / 2f,
            right.transform.position.x - right.transform.lossyScale.x / 2f,
            bottom.transform.position.y + bottom.transform.lossyScale.y / 2f,
            top.transform.position.y - top.transform.lossyScale.y / 2f
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

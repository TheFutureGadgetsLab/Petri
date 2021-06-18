using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Bounds : MonoBehaviour
{
    void Start()
    {
        
    }

    // Update is called once per frame
    void Update()
    {
        
    }

    public List<float> GetBounds()
    {
        return new List<float>(){
            transform.Find("left").transform.position.x,
            transform.Find("right").transform.position.x,
            transform.Find("bottom").transform.position.y,
            transform.Find("top").transform.position.y
        };
    }

    public Vector3 GetRandomPos()
    {
        var bounds = GetBounds();
        return new Vector3(Random.Range(bounds[0], bounds[1]), Random.Range(bounds[2], bounds[3]), 0.0f);
    }
}

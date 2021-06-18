using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class JeffSpawner : MonoBehaviour
{
    GameObject Jeff;
    public uint nJeffs;
    public Vector2 jeffScale = new Vector2(1, 1);

    // Start is called before the first frame update
    void Start()
    {
        List<GameObject> jeffs = new List<GameObject>(){
            Resources.Load<GameObject>("Jeff"),
            Resources.Load<GameObject>("Propulsion")
        };

        var bounds = GameObject.Find("Bounds");

        for (int i = 0; i < nJeffs; i++)
        {
            var jeff = GameObject.Instantiate(jeffs[i % 2], new Vector3(
                Random.Range(
                    bounds.transform.Find("left").transform.position.x,
                    bounds.transform.Find("right").transform.position.x), 
                Random.Range(
                    bounds.transform.Find("bottom").transform.position.y,
                    bounds.transform.Find("top").transform.position.y), 0), Quaternion.identity);
            jeff.transform.localScale = jeffScale;
        }
        
    }

    // Update is called once per frame
    void Update()
    {
        
    }
}

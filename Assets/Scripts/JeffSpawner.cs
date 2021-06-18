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

        var bounds = GameObject.Find("Bounds").GetComponent<Bounds>();

        for (int i = 0; i < nJeffs; i++)
        {
            var jeff = GameObject.Instantiate(
                jeffs[i % 2], 
                bounds.GetRandomPos(),
                Quaternion.identity
            );
            jeff.transform.localScale = jeffScale;
        }
    }

    // Update is called once per frame
    void Update()
    {
        
    }
}
